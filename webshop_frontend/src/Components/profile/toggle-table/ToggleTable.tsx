import ToggleTableRow from "./ToggleTableRow";
import ToggleTableHeader from "./ToggleTableHeader";

export interface ToggleTableHeaderProps {
  text: string[];
}

export interface ToggleTableRowProps {
  row: { text: string[]; toggleOn: boolean };
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
          {props.rows.map((item: ToggleTableRowProps, index: number) => (
            <ToggleTableRow
              rowIndex={index}
              row={item}
              handleClick={props.handleClick}
            />
          ))}
        </tbody>
      </table>
    </>
  );
}
