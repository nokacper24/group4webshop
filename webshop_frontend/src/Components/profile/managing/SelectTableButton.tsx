type SelectTableButtonProps = {
  text: string;
  rowIndex: number;
  action: (index: number) => void;
};

/**
 * Represents a button in a Select Table component.
 *
 * @param props The information about the button.
 * @returns The button component.
 */
export default function SelectTableButton(props: SelectTableButtonProps) {
  return (
    <button
      className={`default-button small-button ${
        props.text.toLowerCase().includes("remove") ? "bg-danger" : ""
      }`}
      onClick={() => props.action(props.rowIndex)}
    >
      {props.text}
    </button>
  );
}
