
pub static ACCORDION_SCRIPT: &str =  "<script>
document.addEventListener('DOMContentLoaded', () => {
    // Get all accordion triggers
    const accordions = document.querySelectorAll('.accordion-trigger');

    accordions.forEach((acc) => {
        acc.addEventListener('click', () => {
            const content = acc.nextElementSibling;
            acc.classList.toggle('is-active');
            content.classList.toggle('is-hidden');
        });
    });
});
</script>";

pub static TABS_SCRIPT: &str = "<script>
        document.addEventListener('DOMContentLoaded', () => {
            // Get all tab elements
            const tabs = document.querySelectorAll('.tabs li');
            const tabsContent = document.querySelectorAll('.tab-content');

            tabs.forEach((tab) => {
                tab.addEventListener('click', () => {
                    const target = tab.dataset.target;

                    // Remove 'is-active' class from all tabs and content
                    tabs.forEach((t) => t.classList.remove('is-active'));
                    tabsContent.forEach((content) => content.classList.add('is-hidden'));

                    // Add 'is-active' class to the selected tab and display the content
                    tab.classList.add('is-active');
                    document.getElementById(target).classList.remove('is-hidden');
                });
            });
        });
    </script>";