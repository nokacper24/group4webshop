import SelectTableCell, { SelectTableCellProps } from "./SelectTableCell";

export type SelectTableRowProps = {
  columns: SelectTableCellProps[];
};

/**
 * Represents a single row in the Select Table component.
 *
 * @param props The columns in the row.
 * @returns The table row component.
 */
export default function SelectTableRow(props: SelectTableRowProps) {
  return (
    <tr>
      {props.columns.map((column, index) => (
        <SelectTableCell key={index} text={column.text} type={column.type} />
      ))}
    </tr>
  );
}
