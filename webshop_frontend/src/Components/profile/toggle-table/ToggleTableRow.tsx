import ToggleButton from "./ToggleButton";
import { ToggleTableRowProps } from "./ToggleTable";

type RowProps = {
  rowIndex: number;
  row: ToggleTableRowProps;
  handleClick: (checked: boolean, id: string) => void;
};

export default function ToggleTableRow(props: RowProps) {
  const row = props.row.row;

  return (
    <>
      <tr>
        {row.text.map((text) => {
          return <td>{text}</td>;
        })}
        <td style={{ width: "6em" }}>
          <ToggleButton
            id={props.rowIndex.toString()}
            handleClick={props.handleClick}
            checked={row.toggleOn}
          />
        </td>
      </tr>
    </>
  );
}
