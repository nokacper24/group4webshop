export type SelectTableHeaderProps = {
  columns: {
    text: string;
  }[];
  toggleSelectAll: () => void;
  selectAll: string;
};

/**
 * The header in the Select Table.
 *
 * @param props The props for the header.
 * @returns The table header component.
 */
export default function SelectTableHeader(props: SelectTableHeaderProps) {
  return (
    <tr>
      <th className="checkbox-column">
        <label>Select</label>
        <input
          type="checkbox"
          checked={props.selectAll == "all"}
          onChange={props.toggleSelectAll}
        />
      </th>
      {props.columns.map((column) => (
        <th key={column.text}>{column.text}</th>
      ))}
      <th></th>
    </tr>
  );
}
