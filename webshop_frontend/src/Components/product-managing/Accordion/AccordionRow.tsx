export function AccordionRow() {
    return (
        <div className="accordion-row">
            <p>Title</p>
            <p>SVG</p>
            <label className="toggle-switch">
                <input type="checkbox" />
                <span className="slider"></span>
            </label>
        </div>
    );
}