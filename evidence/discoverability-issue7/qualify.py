#!/usr/bin/env python3
"""External qualification oracle; not part of the website runtime."""
from pathlib import Path
from html.parser import HTMLParser
import json, subprocess, xml.etree.ElementTree as ET
root = Path(__file__).resolve().parents[2]
evidence = Path(__file__).resolve().parent
dist = root / 'dist'
facts = json.loads((dist/'facts.json').read_text())
class Page(HTMLParser):
    def __init__(self):
        super().__init__(); self.links=[]; self.scripts=[]; self.canonicals=[]; self.images=[]; self.headings=[]
    def handle_starttag(self, tag, attrs):
        a=dict(attrs)
        if tag=='a': self.links.append(a.get('href',''))
        if tag=='script': self.scripts.append(a)
        if tag=='link' and a.get('rel')=='canonical': self.canonicals.append(a['href'])
        if tag=='img': self.images.append(a)
        if tag in ('h1','h2','h3','h4','h5','h6'): self.headings.append(int(tag[1]))
p=Page(); html=(dist/'index.html').read_text(); p.feed(html)
assert p.scripts==[{'type':'application/ld+json'}]
assert p.canonicals==['https://interplane.io/']
assert p.headings.count(1)==1
assert all(b<=a+1 for a,b in zip(p.headings,p.headings[1:]))
assert all(x.get('alt','').strip() for x in p.images)
sitemap=ET.parse(dist/'sitemap.xml').getroot()
assert [x.text for x in sitemap.findall('{*}url/{*}loc')]==p.canonicals
assert not sitemap.findall('{*}url/{*}lastmod')
# Resolve all pinned GitHub source links against the authentic protocol tree once.
full,sha=facts['pin']['full_name'],facts['pin']['sha']
r=subprocess.run(['gh','api',f'repos/{full}/git/trees/{sha}?recursive=1'],capture_output=True,text=True)
assert r.returncode==0,r.stderr
tree=json.loads(r.stdout); assert not tree.get('truncated')
paths={x['path'] for x in tree['tree']}
source_links=[]
for link in p.links:
    marker=f'/blob/{sha}/'
    if marker in link:
        path=link.split(marker,1)[1]
        assert path in paths,('broken pinned path',path)
        source_links.append(link)
(evidence/'protocol-tree.json').write_text(r.stdout+'\n')
(evidence/'facts.json').write_text(json.dumps(facts,indent=2)+'\n')
# Exercise the actual CLI verifier, not a mocked callback.
index=dist/'index.html'; original=index.read_bytes(); outcomes=[]
try:
    candidates=[('valid',html,0),('executable-script',html+'<SCRIPT>alert(1)</SCRIPT>',1),
      ('wrong-canonical',html.replace('rel="canonical" href="https://interplane.io/"','rel="canonical" href="https://wrong.example/"'),1),
      ('invalid-json',html.replace('"@context":"https://schema.org"','"@context": INVALID'),1),
      ('missing-local-link',html.replace('href="facts.json"','href="missing.json"'),1)]
    for name,text,expected in candidates:
        index.write_text(text)
        r=subprocess.run([str(root/'target/debug/site-builder'),'verify',str(dist)],capture_output=True,text=True)
        assert r.returncode==expected,(name,r.returncode,r.stdout,r.stderr)
        outcomes.append(dict(case=name,exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
finally:index.write_bytes(original)
result=dict(schema='interplane-discoverability-qualification/1',protocol_sha=sha,heading_sequence=p.headings,
            pinned_source_links=source_links,cli_controls=outcomes,local_only=True,
            limitation='Single current route; issue6 route expansion, deployed HTTP and organic ranking are not qualified here.')
(evidence/'qualification.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
