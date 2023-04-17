import ToggleButton from "./ToggleButton";
import { ToggleTableRowProps } from "./ToggleTable";

type RowProps = {
  rowIndex: number;
  row: ToggleTableRowProps;
};

export default function ToggleTableRow(props: RowProps) {
  const handleClick = () => {
    console.log("Click");
  };

  const row = props.row.row;

  return (
    <>
      <tr>
        <td>{row.text}</td>
        <td style={{ width: "6em" }}>
          <ToggleButton
            id={props.rowIndex.toString()}
            handleClick={handleClick}
            checked={row.toggleOn}
          />
        </td>
      </tr>
    </>
  );
}
