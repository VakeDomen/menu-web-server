let selectedFile = null;
let previewDiv;
let parseButton;
let parseManualButton;
let galleryButton;
let galleryInput;


function setParsingUtilRefs(previewDivRef, parseButtonRef, parseManualButtonRef, galleryButtonRef, galleryInputRef) {
    previewDiv = previewDivRef;
    parseButton = parseButtonRef;
    parseManualButton = parseManualButtonRef;
    galleryButton = galleryButtonRef;
    galleryInput = galleryInputRef;

    parseManualButton.addEventListener('click', () => skipParsing())
    parseButton.addEventListener('click', (event) => parseImageWithLLM(event, parseButton));
    galleryButton.addEventListener('click', () => galleryInput.click());
    galleryInput.addEventListener('change', () => handleImageSelection(galleryInput));
}

function skipParsing() {
    prefillForm({
        restaurant: "",
        contact: "",
        theme: "",
        menu: {
            categories: []
        }
    });
    if (categoriesContainer.childElementCount === 0) {
        addCategory();
    }
    toMenu();
}

async function parseImageWithLLM(event) {
    event.preventDefault();
    const file = selectedFile;
    if (!file) {
        alert('Please select an image.');
        return;
    }

    parseButton.disabled = true;
    parseButton.textContent = 'Parsing...';

    toLoading();

    try {
        // Convert image to base64
        const base64Image = await toBase64(file);

        // Send image to server for parsing
        const response = await fetch('/parse_image', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ image: base64Image }),
        });

        if (!response.ok) {
            throw new Error('Error parsing image.');
        }

        const data = await response.json();

        // Prefill the form with parsed data
        prefillForm(data);

        parseButton.textContent = 'Parse';
        parseButton.disabled = false;
        toMenu();
    } catch (error) {
        console.error(error);
        alert('An error occurred while parsing the image.');
        parseButton.textContent = 'Parse';
        parseButton.disabled = false;
        toUpload();
    }
}


// Helper function to convert image to base64
function toBase64(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = function (e) {
            resolve(e.target.result.split(',')[1]); // Remove data:image/*;base64,
        };
        reader.onerror = function (error) {
            reject(error);
        };
        reader.readAsDataURL(file);
    });
}

// Handle image selection
function handleImageSelection(input) {
    const file = input.files[0];
    if (file) {
        // Show the image thumbnail
        const reader = new FileReader();
        reader.onload = function (e) {
            previewDiv.src = e.target.result;
            previewDiv.style.display = 'block';
        };
        reader.readAsDataURL(file);
        parseButton.disabled = false;
        // Store the selected file
        selectedFile = file;
    } else {
        previewDiv.src = '#';
        previewDiv.style.display = 'none';
        parseButton.disabled = true;
        selectedFile = null;
    }
}