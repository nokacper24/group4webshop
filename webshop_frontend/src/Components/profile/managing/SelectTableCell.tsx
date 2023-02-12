export type SelectTableCellProps = {
  text: string;
  type: string;
};

export default function SelectTableCell(props: SelectTableCellProps) {
  let element;
  switch (props.type.toLowerCase()) {
    case "checkbox":
      element = <input type="checkbox" />;
      break;
    case "button":
      element = (
        <button className="default-button small-button">{props.text}</button>
      );
      break;
    case "danger-button":
      element = (
        <button className="default-button danger small-button">
          {props.text}
        </button>
      );
      break;
    default:
      element = <span>{props.text}</span>;
  }

  return <td>{element}</td>;
}
