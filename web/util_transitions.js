let imageUploadDiv;
let loadingDiv;
let menuDiv;

function setTransitionRefs(imageUploadDivRef, loadingDivRef, menuDivRef) {
    imageUploadDiv = imageUploadDivRef;
    loadingDiv = loadingDivRef;
    menuDiv = menuDivRef
}

function toLoading() {
    menuDiv.style.display = "none";
    imageUploadDiv.style.display = "none";
    loadingDiv.style.display = "block";
}

function toMenu() {
    loadingDiv.style.display = "none";
    imageUploadDiv.style.display = "none";
    menuDiv.style.display = "block";
}

function toUpload() {
    loadingDiv.style.display = "none";
    imageUploadDiv.style.display = "block";
    menuDiv.style.display = "none";
}