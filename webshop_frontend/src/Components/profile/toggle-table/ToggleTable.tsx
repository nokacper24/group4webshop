import ToggleTableRow from "./ToggleTableRow";
import ToggleTableHeader from "./ToggleTableHeader";

export interface ToggleTableHeaderProps {
  text: string[];
}

export interface ToggleTableRowProps {
  row: { text: string; toggleOn: boolean };
}

export interface ToggleTableProps {
  headers: ToggleTableHeaderProps;
  rows: ToggleTableRowProps[];
}

/**
 * Represents a Toggle Table component.
 * Each row in the table will have some text and
 * a toggle button (checkbox) that can either be on or off.
 *
 * @returns A Toggle Table component.
 */
export default function ToggleTable() {
  const headers: ToggleTableHeaderProps = {
    text: ["License", "Active"],
  };

  const row1: ToggleTableRowProps = {
    row: {
      text: "License 1",
      toggleOn: false,
    },
  };

  const row2: ToggleTableRowProps = {
    row: {
      text: "License 2",
      toggleOn: true,
    },
  };

  const placeholder: ToggleTableProps = {
    headers: headers,
    rows: [row1, row2],
  };

  const handleSave = () => {
    console.log("Saving...");
  };

  return (
    <>
      <table className="toggle-table">
        <ToggleTableHeader text={placeholder.headers.text} />
        <tbody>
          {placeholder.rows.map((item: ToggleTableRowProps, index: number) => (
            <ToggleTableRow rowIndex={index} row={item} />
          ))}
        </tbody>
      </table>
      <button className="default-button" onClick={handleSave}>
        Save
      </button>
    </>
  );
}
