const navbar = document.querySelector('nav');
const main = document.querySelector('main');
const children = main.children;
const toc = {};

let cursorH1, cursorH2;
for (let i = 0; i < children.length; i++) {
    const child = children[i];
    const tag = child.tagName.toLowerCase();
    if (['h1', 'h2', 'h3'].indexOf(tag) === -1) continue;
    const text = child.innerText;
    const id = encodeURI(text.replace(/\s/g, '-'));
    child.setAttribute('id', id);
    if (tag === 'h1') {
        toc[id] = {
            id, text,
            item: {},
        };
        cursorH1 = toc[id].item;
    }
    if (tag === 'h2') {
        cursorH1[id] = {
            id, text,
            item: {},
        };
        cursorH2 = cursorH1[id].item;
    }
    if (tag === 'h3') {
        cursorH2[id] = { id, text};
    }
}

console.log(JSON.stringify(toc, null, 2));

for (let h1 in toc) {
    const header = toc[h1];
    const dl = document.createElement('dl');
    const dt = document.createElement('dt');
    const a = document.createElement('a');
    a.setAttribute('href', `#${header.id}`);
    a.innerText = header.text;
    dt.appendChild(a);
    dl.appendChild(dt);
    console.log(header.item);
    if (Object.keys(header.item).length > 0) {
        const dd = document.createElement('dd');
        const dl2 = document.createElement('dl');
        for (let h2 in header.item) {
            const header2 = header.item[h2];
            const dt2 = document.createElement('dt');
            const a2 = document.createElement('a');
            a2.setAttribute('href', `#${header2.id}`);
            a2.innerText = header2.text;
            dt2.appendChild(a2);
            dl2.appendChild(dt2);
            dd.appendChild(dl2);
            dl.appendChild(dd);
            if (Object.keys(header2.item).length > 0) {
                const dd2 = document.createElement('dd');
                const dl3 = document.createElement('dl');
                for (let h3 in header2.item) {
                    const header3 = header2.item[h3];
                    const dt3 = document.createElement('dt');
                    const a3 = document.createElement('a');
                    a3.setAttribute('href', `#${header3.id}`);
                    a3.innerText = header3.text;
                    dt3.appendChild(a3);
                    dl3.appendChild(dt3);
                }
                dd2.appendChild(dl3);
                dl2.appendChild(dd2);
            }
        }
    }
    navbar.appendChild(dl);
}
