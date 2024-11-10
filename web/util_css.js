
function longUrl(url) {
    return "/static/theme_" + url + ".css"
}


function loadCss(url) {
    // Check if the CSS is already loaded to prevent duplicates
    if (document.querySelector(`link[href="${url}"]`)) {
        console.warn(`CSS file "${url}" is already loaded.`);
        return;
    }

    // Create a new link element
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.type = 'text/css';
    link.href = url;
    link.id = `css-${encodeURIComponent(url)}`; // Optional: Assign an ID for easier reference

    // Optionally, handle load and error events
    link.onload = () => {
        console.log(`CSS file "${url}" has been loaded successfully.`);
        loadedCssFiles.push(url)
    };

    link.onerror = () => {
        console.error(`Failed to load CSS file "${url}".`);
    };

    // Append the link element to the head
    document.head.appendChild(link);
}


function unloadCss(url) {
    console.log("unloading " + url + "...")
    // Select all link elements with the matching href
    const links = document.querySelectorAll(`link[href="${url}"]`);

    if (links.length === 0) {
        console.warn(`No CSS file found with URL "${url}".`);
        return;
    }

    // Remove each found link element from the head
    links.forEach(link => {
        if (link.parentNode) {
            link.parentNode.removeChild(link);
            console.log(`CSS file "${url}" has been unloaded.`);
        }
    });


}