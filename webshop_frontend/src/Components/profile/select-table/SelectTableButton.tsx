type SelectTableButtonProps = {
  text: string;
  id: string;
  action: (id: string) => void;
};

export const dangerWords = ["remove", "invalidate"];

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
        dangerWords.some((el) => props.text.toLowerCase().includes(el))
          ? "bg-danger"
          : ""
      }`}
      onClick={() => props.action(props.id)}
    >
      {props.text}
    </button>
  );
}
