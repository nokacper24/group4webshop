import ToggleTableRow from "./ToggleTableRow";
import ToggleTableHeader from "./ToggleTableHeader";

export interface ToggleTableHeaderProps {
  text: string[];
}

export interface ToggleTableRowProps {
  row: { id: string; text: string[]; toggleOn: boolean };
}

export interface ToggleTableProps {
  headers: ToggleTableHeaderProps;
  rows: ToggleTableRowProps[];
  handleClick: (checked: boolean, id: string) => void;
}

/**
 * Represents a Toggle Table component.
 * Each row in the table will have some text and
 * a toggle button (checkbox) that can either be on or off.
 *
 * @returns A Toggle Table component.
 */
export default function ToggleTable(props: ToggleTableProps) {
  return (
    <>
      <table className="toggle-table">
        <ToggleTableHeader text={props.headers.text} />
        <tbody>
          {props.rows.map((item: ToggleTableRowProps) => (
            <ToggleTableRow
              id={item.row.id}
              row={item}
              handleClick={props.handleClick}
            />
          ))}
        </tbody>
      </table>
    </>
  );
}
