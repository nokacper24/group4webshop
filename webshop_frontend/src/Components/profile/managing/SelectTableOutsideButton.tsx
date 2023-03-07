import { dangerWords } from "./SelectTableButton";

type SelectTableButtonProps = {
  text: string;
  indices: number[];
  action: (indices: number[]) => void;
  clearSelected: () => void;
};

/**
 * Represents a button in a Select Table component.
 *
 * @param props The information about the button.
 * @returns The button component.
 */
export default function SelectTableOutsideButton(
  props: SelectTableButtonProps
) {
  const handleClick = () => {
    props.action(props.indices);
    props.clearSelected();
  };

  return (
    <button
      className={`default-button small-button ${
        dangerWords.some((el) => props.text.toLowerCase().includes(el))
          ? "bg-danger"
          : ""
      }`}
      onClick={() => handleClick()}
    >
      {props.text}
    </button>
  );
}
