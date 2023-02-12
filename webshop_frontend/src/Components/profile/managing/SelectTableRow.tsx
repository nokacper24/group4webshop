import SelectTableCell, { SelectTableCellProps } from "./SelectTableCell";

export type SelectTableRowProps = {
  columns: SelectTableCellProps[];
};

export default function SelectTableRow(props: SelectTableRowProps) {
  return (
    <tr>
      {props.columns.map((column, index) => (
        <SelectTableCell key={index} text={column.text} type={column.type} />
      ))}
    </tr>
  );
}
