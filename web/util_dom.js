function createDiv(classList) {
    const div = document.createElement('div');
    div.classList.add(...classList);
    return div;
}

function createLabel(textContent, classList=[]) {
    const label = document.createElement('label');
    label.classList.add(...(classList.length ? classList : ['label']));
    label.textContent = textContent;
    return label
}

function createButton(text, classList) {
    const button = document.createElement('button');
    button.type = 'button';
    button.classList.add(...classList);
    button.textContent = text;
    return button;

}

function createInputField(labelText, inputName, inputValue) {
    const field = createDiv(['field']);
    const label = createLabel(labelText);
    const control = createDiv(['control']);
    const input = document.createElement('input');
    input.classList.add('input');
    input.type = 'text';
    input.name = inputName;
    input.value = inputValue;

    control.appendChild(input);
    field.appendChild(label);
    field.appendChild(control);

    return field;
}


function createTextareaField(labelText, textareaName, textareaValue) {
    const field = createDiv(['field']);
    const label = createLabel(labelText);
    const control = createDiv(['control']);
    const textarea = document.createElement('textarea');
    textarea.classList.add('textarea');
    textarea.name = textareaName;
    textarea.value = textareaValue;

    control.appendChild(textarea);
    field.appendChild(label);
    field.appendChild(control);

    return field;
}

function decodeHtmlEntities(str) {
    const txt = document.createElement("textarea");
    txt.innerHTML = str;
    return txt.value;
}