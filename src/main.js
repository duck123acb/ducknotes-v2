const { invoke } = window.__TAURI__.core;

let parent;

const exampleMD = `
# Duck Notes
This is a small note taking app for the web.  
For now, this use ~~editing~~ plain text documents quickly!  
It is a local saving system  
**Deploys *from* Netlify**
`;

function renderElement(element) {
  const { element_type, content, sub_elements } = element;

  const el = document.createElement(element_type);

  el.textContent = content;

  sub_elements.forEach((sub_element) => {
    const subEl = renderElement(sub_element);
    el.appendChild(subEl);
  });

  return el;
}

async function calculateNode(content) {
  let elements = await invoke("parse_md", { content: content });

  elements.forEach((element) => {
    const el = renderElement(element);

    parent.appendChild(el);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  parent = document.querySelector("#parent");
  calculateNode(exampleMD);
});

/*

  [
        Element {
            element_type: "span",
            content: "",
            sub_elements: [Element {
                element_type: "span",
                content: "",
                sub_elements: [Element {
                    element_type: "span",
                    content: "",
                    sub_elements: [],
                }],
            }],
        },
        Element {
            element_type: "span",
            content: "",
            sub_elements: [Element {
                element_type: "h1",
                content: "",
                sub_elements: [Element {
                    element_type: "span",
                    content: "Duck Notes",
                    sub_elements: [],
                }],
            }],
        },
        Element {
            element_type: "span",
            content: "",
            sub_elements: [
                Element {
                    element_type: "span",
                    content: "",
                    sub_elements: [Element {
                        element_type: "span",
                        content: "This is a small note taking app for the web  ",
                        sub_elements: [],
                    }],
                },
                Element {
                    element_type: "br",
                    content: "",
                    sub_elements: [],
                },
            ],
        },
        Element {
            element_type: "span",
            content: "",
            sub_elements: [
                Element {
                    element_type: "span",
                    content: "",
                    sub_elements: [
                        Element {
                            element_type: "span",
                            content: "For now, this use ",
                            sub_elements: [],
                        },
                        Element {
                            element_type: "del",
                            content: "",
                            sub_elements: [Element {
                                element_type: "span",
                                content: "editing",
                                sub_elements: [],
                            }],
                        },
                    ],
                },
                Element {
                    element_type: "br",
                    content: "",
                    sub_elements: [],
                },
            ],
        },
        Element {
            element_type: "span",
            content: "",
            sub_elements: [
                Element {
                    element_type: "span",
                    content: "",
                    sub_elements: [Element {
                        element_type: "span",
                        content: "It is a local saving system  ",
                        sub_elements: [],
                    }],
                },
                Element {
                    element_type: "br",
                    content: "",
                    sub_elements: [],
                },
            ],
        },
        Element {
            element_type: "span",
            content: "",
            sub_elements: [Element {
                element_type: "span",
                content: "",
                sub_elements: [Element {
                    element_type: "strong",
                    content: "",
                    sub_elements: [
                        Element {
                            element_type: "span",
                            content: "Deploys ",
                            sub_elements: [],
                        },
                        Element {
                            element_type: "em",
                            content: "",
                            sub_elements: [Element {
                                element_type: "span",
                                content: "from",
                                sub_elements: [],
                            }],
                        },
                    ],
                }],
            }],
        },
    ]


  */
