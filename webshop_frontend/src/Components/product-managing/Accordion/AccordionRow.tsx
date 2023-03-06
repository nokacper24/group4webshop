export function AccordionRow() {
    return (
        <div className="accordion-row">
            <p>Title</p>
            <p>SVG</p>
            <label className="toggle-switch">
                <input type="checkbox" />
                <span className="slider"></span>
            </label>
            <button className="accordion-remove-button">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><title>Remove</title><path fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="32" d="M400 256H112"/></svg>
            </button>
        </div>
    );
}