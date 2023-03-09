export type AccordionRowProps = {
  title: string;
  id: number;
  editFunction: (id: number) => void;
  removeFunction: (id: number) => void;
};

export function AccordionRow(props: AccordionRowProps) {
  if (props.title.length <= 0) {
    props.title = "Title";
  }
  return (
    <div className="accordion-row">
      <p>{props.title}</p>
      <button
        className="accordion-edit-button"
        onClick={() => props.editFunction(props.id)}
      >
        <svg
          className="accordion-button-icon"
          version="1.2"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 200 200"
        >
          <title>Edit</title>
          <path
            id="Layer"
            className="pencil-s0"
            d="m145.1 29l15.9-15.8c3.3-3.3 7.7-5.2 12.4-5.2 4.7 0 9.1 1.9 12.4 5.2 3.3 3.3 5.2 7.7 5.2 12.4 0 4.7-1.9 9.1-5.2 12.4l-134.8 134.9q-1.9 1.8-3.9 3.4-2.1 1.7-4.3 3-2.3 1.3-4.7 2.4-2.4 1-4.9 1.8l-25.2 7.5 7.5-25.2q0.8-2.5 1.8-4.9 1.1-2.4 2.4-4.7 1.3-2.2 3-4.3 1.6-2 3.4-3.9c0 0 119-119 119-119zm0 0l24.8 24.7"
          />
          <path
            id="Shape 1"
            className="pencil-s1"
            d="m36 165m-2-21m15 30c-23-25.6-23-26-23-26m19-2c100.7-99.4 101-99 101-99m-93 104c99.4-97.3 99-97 99-97"
          />
          <path
            id="Layer 1"
            className="pencil-s0"
            d="m25 148c24 26.3 24 26 24 26"
          />
        </svg>
      </button>
      <label className="toggle-switch">
        <input type="checkbox" />
        <span className="slider"></span>
      </label>
      <button
        className="accordion-remove-button"
        onClick={() => props.removeFunction(props.id)}
      >
        <svg
          className="accordion-button-icon"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 512 512"
        >
          <title>Remove</title>
          <path
            fill="none"
            stroke="currentColor"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="32"
            d="M400 256H112"
          />
        </svg>
      </button>
    </div>
  );
}
