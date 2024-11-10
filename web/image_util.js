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
            imagePreview.src = e.target.result;
            imagePreview.style.display = 'block';
        };
        reader.readAsDataURL(file);
        parseButton.disabled = false;
        // Store the selected file
        selectedFile = file;
    } else {
        imagePreview.src = '#';
        imagePreview.style.display = 'none';
        parseButton.disabled = true;
        selectedFile = null;
    }
}