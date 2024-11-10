let categoriesList = [];
let loadedCssFiles = [];

let menuForm;
let categoriesContainer;
let themeInput;
let addCategoryButton;

function setMenuUtilRefs(menuFormRef, categoriesContainerRef, themeInputRef, addCategoryButtonRef) {
    menuForm = menuFormRef;
    categoriesContainer = categoriesContainerRef;
    themeInput = themeInputRef;
    addCategoryButton = addCategoryButtonRef;

    addCategoryButton.addEventListener('click', () => addCategory());
}

function prefillForm(parsedData) {
    // Clear existing categories
    categoriesContainer.innerHTML = '';

    // Reset category index
    categoryIndex = 0;

    // Set restaurant name, contact, and theme
    menuForm.elements['restaurant'].value = parsedData.restaurant || '';
    menuForm.elements['contact'].value = parsedData.contact || '';
    menuForm.elements['theme'].value = parsedData.theme || '';

    // Iterate over categories
    parsedData.menu.categories.forEach((category) => {
        addCategory(category);
    });
    
    themeInput.addEventListener("change", (event) => {
        while (loadedCssFiles.length > 0) {
            let url = loadedCssFiles.pop();
            unloadCss(url)
        }
        
        loadCss(longUrl(event.target.value))
    })
}

function moveItemToCategory(itemBox, newCategoryIndex) {
    const oldCategoryIndex = parseInt(itemBox.dataset.categoryIndex);
    const itemIndex = parseInt(itemBox.dataset.itemIndex);

    // Remove itemBox from old category's items container
    const oldCategory = categoriesList.find(cat => cat.index === oldCategoryIndex);
    oldCategory.itemsContainer.removeChild(itemBox);

    // Update the categoryIndex in itemBox
    itemBox.dataset.categoryIndex = newCategoryIndex;

    // Add itemBox to new category's items container
    const newCategory = categoriesList.find(cat => cat.index === newCategoryIndex);
    newCategory.itemsContainer.appendChild(itemBox);

    // Reindex items in old category
    reindexItemsInCategory(oldCategoryIndex);

    // Reindex items in new category
    reindexItemsInCategory(newCategoryIndex);
}

function reindexItemsInCategory(categoryIndex) {
    const category = categoriesList.find(cat => cat.index === categoryIndex);
    const itemsContainer = category.itemsContainer;
    const itemBoxes = itemsContainer.querySelectorAll('.item-box');
    itemBoxes.forEach((itemBox, index) => {
        itemBox.dataset.itemIndex = index;
        // Update the name attributes
        updateItemNameAttributes(itemBox, categoryIndex, index);
    });
}


function addCategory(categoryData = null) {
    const i = categoryIndex++;

    const categoryBox = createDiv(['box', 'category-box']);
    categoryBox.dataset.categoryIndex = i; // Store the category index

    // Category Header (Accordion)
    const categoryHeader = createDiv(['category-header', 'is-flex', 'is-align-items-center']);
    const categoryTitle = document.createElement('span');
    categoryTitle.textContent = categoryData ? categoryData.name || `Category ${i + 1}` : `Category ${i + 1}`;
    categoryTitle.style.flexGrow = '1';

    // Category Name Input
    const categoryNameInput = document.createElement('input');
    categoryNameInput.type = 'hidden';
    categoryNameInput.name = `category_name_${i}`;
    categoryNameInput.value = categoryData ? categoryData.name || '' : '';

    // Remove Category Button
    const removeCategoryButton = createButton('Remove Category', ['button', 'is-small', 'is-danger', 'remove-button']);
    removeCategoryButton.addEventListener('click', () => {
        categoriesContainer.removeChild(categoryBox);
        updateCategoryOptionsInItems();
    });

    // Toggle category content visibility
    categoryHeader.addEventListener('click', () => {
        categoryContent.classList.toggle('active');
    });

    categoryHeader.appendChild(categoryTitle);
    categoryHeader.appendChild(removeCategoryButton);
    categoryHeader.appendChild(categoryNameInput);

    // Category Content
    const categoryContent = createDiv(['category-content']);

    // Edit Category Name Field
    const editCategoryNameField = createDiv(['field']);
    const editCategoryNameLabel = createLabel('Category Name');
    const editCategoryNameControl = createDiv(['control']);
    const editCategoryNameInput = document.createElement('input');
    editCategoryNameInput.classList.add('input');
    editCategoryNameInput.type = 'text';
    editCategoryNameInput.value = categoryData ? categoryData.name || '' : '';
    editCategoryNameInput.addEventListener('input', () => {
        categoryNameInput.value = editCategoryNameInput.value;
        categoryTitle.textContent = editCategoryNameInput.value || `Category ${i + 1}`;
    });

    editCategoryNameControl.appendChild(editCategoryNameInput);
    editCategoryNameField.appendChild(editCategoryNameLabel);
    editCategoryNameField.appendChild(editCategoryNameControl);

    categoryContent.appendChild(editCategoryNameField);

    // Items Container
    const itemsContainer = createDiv(['items-container']);

    categoryContent.appendChild(itemsContainer);

    // Add Item Button
    const buttonContainer = createDiv(['items-container']);
    categoryContent.appendChild(buttonContainer);

    const addItemButton = createButton('Add Item', ['button', 'is-small', 'is-link', 'is-light', 'add-item-button']);
    addItemButton.addEventListener('click', () => {
        addItem(i, itemsContainer);
    });

    buttonContainer.appendChild(addItemButton);

    // Append existing items
    if (categoryData && categoryData.items) {
        categoryData.items.forEach((item) => {
            addItem(i, itemsContainer, item);
        });
    }

    categoryBox.appendChild(categoryHeader);
    categoryBox.appendChild(categoryContent);
    categoriesContainer.appendChild(categoryBox);
    // Add the category to the categoriesList
    categoriesList.push({
        index: i,
        nameInput: categoryNameInput,
        titleElement: categoryTitle,
        itemsContainer: itemsContainer,
        categoryBox: categoryBox
    });

    // Update category options in all items
    updateCategoryOptionsInItems();

    // Rest of your existing code...
}
function updateItemNameAttributes(itemBox, categoryIndex, itemIndex) {
    // Update the name attributes of inputs in the itemBox
    const inputs = itemBox.querySelectorAll('input, textarea');
    inputs.forEach(input => {
        if (input.name.startsWith('item_name_') || input.name.startsWith('item_price_') || input.name.startsWith('item_description_')) {
            const nameParts = input.name.split('_');
            const fieldType = nameParts[0] + '_' + nameParts[1];
            input.name = `${fieldType}_${categoryIndex}_${itemIndex}`;
        }
    });
}

function updateCategoryOptionsInItems() {
    // For each item in each category, update the category options
    categoriesList.forEach(cat => {
        const itemsContainer = cat.itemsContainer;
        const itemBoxes = itemsContainer.querySelectorAll('.item-box');
        itemBoxes.forEach(itemBox => {
            const moveCategorySelect = itemBox.querySelector('select');
            const currentCategoryIndex = parseInt(itemBox.dataset.categoryIndex);
            // Clear existing options
            moveCategorySelect.innerHTML = '';
            // Populate category options
            categoriesList.forEach(catOption => {
                const option = document.createElement('option');
                option.value = catOption.index;
                option.textContent = catOption.nameInput.value || `Category ${catOption.index + 1}`;
                if (catOption.index === currentCategoryIndex) {
                    option.selected = true;
                }
                moveCategorySelect.appendChild(option);
            });
        });
    });
}


function addItem(currentCategoryIndex, itemsContainer, itemData = null) {
    const itemIndex = itemsContainer.childElementCount;

    const itemBox = createDiv(['box', 'item-box', 'is-description']);
    itemBox.dataset.categoryIndex = currentCategoryIndex;
    itemBox.dataset.itemIndex = itemIndex;

    // Remove Item Button
    const removeItemRow = createDiv(['has-text-right'])
    const removeItemButton = createButton('Remove Item', ['button', 'is-small', 'is-danger', 'remove-button']);
    removeItemRow.appendChild(removeItemButton)

    removeItemButton.addEventListener('click', () => {
        itemsContainer.removeChild(itemBox);
        // Reindex items in the category
        reindexItemsInCategory(currentCategoryIndex);
    });

    // Move to Category Dropdown
    const moveCategoryField = createDiv(['level']);
    const moveCategoryLabel = createLabel("Move to Category", ['level-left'])
    const moveCategoryControl = createDiv(['level-right', 'mt-0']);
    const moveCategorySelect = document.createElement('select');

    // Populate category options
    categoriesList.forEach(cat => {
        const option = document.createElement('option');
        option.value = cat.index;
        option.textContent = cat.nameInput.value || `Category ${cat.index + 1}`;
        if (cat.index === currentCategoryIndex) {
            option.selected = true;
        }
        moveCategorySelect.appendChild(option);
    });

    // Event listener for changing category
    moveCategorySelect.addEventListener('change', () => {
        const newCategoryIndex = parseInt(moveCategorySelect.value);
        if (newCategoryIndex !== currentCategoryIndex) {
            moveItemToCategory(itemBox, newCategoryIndex);
        }
    });

    moveCategoryControl.appendChild(moveCategorySelect);
    moveCategoryField.appendChild(moveCategoryLabel);
    moveCategoryField.appendChild(moveCategoryControl);

    // Item Name
    const itemNameField = createInputField(`Item Name`, `item_name_${currentCategoryIndex}_${itemIndex}`, itemData ? itemData.name || '' : '');

    // Item Price
    const itemPriceField = createInputField(`Item Price`, `item_price_${currentCategoryIndex}_${itemIndex}`, itemData ? itemData.price || '' : '');

    // Item Description
    const itemDescField = createTextareaField(`Item Description`, `item_description_${currentCategoryIndex}_${itemIndex}`, itemData ? itemData.description || '' : '');

    itemBox.appendChild(removeItemRow);
    itemBox.appendChild(itemNameField);
    itemBox.appendChild(itemPriceField);
    itemBox.appendChild(itemDescField);
    itemBox.appendChild(moveCategoryField); 

    itemsContainer.appendChild(itemBox);
}

