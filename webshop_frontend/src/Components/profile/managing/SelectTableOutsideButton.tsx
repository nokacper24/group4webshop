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
        props.text.toLowerCase().includes("remove") ? "bg-danger" : ""
      }`}
      onClick={() => handleClick()}
    >
      {props.text}
    </button>
  );
}
