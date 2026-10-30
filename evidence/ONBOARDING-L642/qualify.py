#!/usr/bin/env python3
"""Qualify actual generated routes and replay the literal rendered Quick Start.
Run at repository root after cargo build/build-dist. No deployment or model use.
"""
import gzip, hashlib, json, os, re, shutil, signal, subprocess, tempfile
from html.parser import HTMLParser
from pathlib import Path
from typing import Any
ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
DIST=ROOT/'dist'
BINARY=ROOT/'target/debug/site-builder'
ROUTES=['/','/quick-start/','/concepts/','/adapter/','/conformance/','/status/']

def save(name,data):
    if isinstance(data,str): data=data.encode()
    (OUT/(name+'.gz')).write_bytes(gzip.compress(data,mtime=0))
    return {'file':name+'.gz','raw_sha256':hashlib.sha256(data).hexdigest(),'raw_bytes':len(data)}
def run(args,cwd=ROOT,timeout=180):
    proc=subprocess.Popen(args,cwd=cwd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,start_new_session=True)
    try: raw=proc.communicate(timeout=timeout)[0]
    except subprocess.TimeoutExpired:
        os.killpg(proc.pid,signal.SIGTERM)
        try: raw=proc.communicate(timeout=5)[0]
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid,signal.SIGKILL); raw=proc.communicate()[0]
        return 124,raw
    return proc.returncode,raw
class Page(HTMLParser):
    def __init__(self,text):
        super().__init__(convert_charrefs=True)
        self.links=[]; self.codes=[]; self.code=None; self.title=False; self.title_text=''; self.meta=[]; self.canonical=[]
        self.feed(text)
    def handle_starttag(self,tag,attrs):
        attrs=dict(attrs)
        if tag=='a': self.links.append(attrs.get('href'))
        if tag=='meta': self.meta.append(attrs)
        if tag=='link' and attrs.get('rel')=='canonical': self.canonical.append(attrs.get('href'))
        if tag=='title': self.title=True
        if tag=='code': self.code=[attrs,'']
    def handle_data(self,data):
        if self.title:self.title_text+=data
        if self.code is not None:self.code[1]+=data
    def handle_endtag(self,tag):
        if tag=='title':self.title=False
        if tag=='code' and self.code is not None:self.codes.append(self.code);self.code=None

def main():
    manifest:dict[str,Any]={'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
              'binary_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest(),
              'facts_sha256':hashlib.sha256((DIST/'facts.json').read_bytes()).hexdigest(),
              'scope':'static site and pinned scripted CPU reference; no deployment, model, GPU or general security qualification'}
    shutil.copyfile(DIST/'facts.json',OUT/'facts-snapshot.json')
    pages={route:Page((DIST/route.strip('/')/'index.html').read_text()) for route in ROUTES}
    for route,p in pages.items():
        assert p.canonical==['https://interplane.io'+route],(route,p.canonical)
        assert all(r in p.links for r in ROUTES),(route,p.links)
        assert [m for m in p.meta if m.get('name')=='description'],route
    assert len({p.title_text for p in pages.values()})==6
    assert len({next(m['content'] for m in p.meta if m.get('name')=='description') for p in pages.values()})==6
    manifest['routes']=ROUTES
    with tempfile.TemporaryDirectory(prefix='L642-controls-',dir=os.environ['TMPDIR']) as temp:
        dist=Path(temp)/'dist';shutil.copytree(DIST,dist)
        rc,raw=run([str(BINARY),'verify',str(dist)]);assert rc==0,raw
        controls=[]
        for route in ROUTES:
            file=dist/route.strip('/')/'index.html';original=file.read_text()
            url='https://interplane.io'+route
            mutations={
                'wrong-canonical':original.replace(f'rel="canonical" href="{url}"','rel="canonical" href="https://wrong.example/"'),
                'wrong-title':original.replace('<title>INTERPLANE:','<title>WRONG:'),
                'wrong-json-ld':original.replace('SoftwareSourceCode','FakeSoftware'),
                'duplicate-head-canonical':original.replace('</head>',"<link rel='canonical' href='https://wrong.example/'></head>"),
                'duplicate-body-canonical':original.replace('</body>',"<LINK REL='canonical' HREF='https://wrong.example/'></body>"),
                'duplicate-head-image':original.replace('</head>',"<meta property='og:image' content='https://wrong.example/a.webp'></head>"),
                'duplicate-body-image':original.replace('</body>',"<META PROPERTY='og:image' CONTENT='https://wrong.example/a.webp'></body>"),
                'missing-route-target':original.replace('</body>','<a href="/missing-route/">missing</a></body>'),
                'missing-cross-page-fragment':original.replace('</body>','<a href="/quick-start/#missing-id">missing</a></body>'),
                'parent-traversal':original.replace('</body>','<a href="../facts.json">unsafe</a></body>'),
                'protocol-relative-target':original.replace('</body>','<a href="//wrong.example/">unsafe</a></body>'),
            }
            for name,changed in mutations.items():
                assert changed!=original,(route,name)
                try:
                    file.write_text(changed);rc,raw=run([str(BINARY),'verify',str(dist)])
                    controls.append({'route':route,'control':name,'exit':rc,'diagnostics':raw.decode(errors='replace')})
                    assert rc!=0,(route,name,'accepted')
                finally:file.write_text(original)
            saved=file.with_name('index.saved');file.rename(saved)
            try:
                rc,raw=run([str(BINARY),'verify',str(dist)])
                controls.append({'route':route,'control':'missing-page','exit':rc,'diagnostics':raw.decode(errors='replace')});assert rc!=0
            finally:saved.rename(file)
        rc,raw=run([str(BINARY),'verify',str(dist)]);assert rc==0,raw
        manifest['controls']={'count':len(controls),'all_refused':all(c['exit']!=0 for c in controls),'raw':save('controls.json',json.dumps(controls,indent=2)+'\n')}
    quick=pages['/quick-start/']; commands=next(text for attrs,text in quick.codes if attrs.get('class')=='language-sh')
    expected=quick.codes[1][1]
    match=re.search(r'git checkout --detach ([0-9a-f]{40})',commands); assert match
    pin=match.group(1)
    (OUT/'rendered-quick-start.sh').write_text(commands+'\n')
    with tempfile.TemporaryDirectory(prefix='L642-rendered-recipe-',dir=os.environ['TMPDIR']) as temp:
        rc,raw=run(['sh','-eu','-c',commands],cwd=Path(temp))
        proof={'exit':rc,'recipe_pin':pin,'expected_final_line':expected,'raw':save('quick-start-replay.log',raw)}
        manifest['quick_start']=proof
        (OUT/'qualification.json').write_text(json.dumps(manifest,indent=2)+'\n')
        assert rc==0,raw.decode(errors='replace')
        actual=subprocess.check_output(['git','rev-parse','HEAD'],cwd=Path(temp)/'interplane',text=True).strip();assert actual==pin
        proof['checked_out_pin']=actual
        lines=raw.decode().strip().splitlines();proof['actual_final_line']=lines[-1];assert lines[-1]==expected,(lines[-1],expected)
        rows=re.findall(r'^c[1-4]\s+(web_fetch|send_email|write_file|append_note)\s+(authorized|denied|requires_approval)\s+(True|False)\s*$',raw.decode(),re.M)
        assert rows==[('web_fetch','authorized','True'),('send_email','denied','False'),('write_file','denied','False'),('append_note','requires_approval','False')],rows
        proof['observed_rows']=rows
    (OUT/'qualification.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(json.dumps(manifest,indent=2))
if __name__=='__main__':main()
