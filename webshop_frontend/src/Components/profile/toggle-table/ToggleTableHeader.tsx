import { ToggleTableHeaderProps } from "./ToggleTable";

export default function ToggleTableHeader(props: ToggleTableHeaderProps) {
  return (
    <>
      <thead>
        <tr>
          {props.text.map((column, index) => (
            <th key={column + index}>{column}</th>
          ))}
        </tr>
      </thead>
    </>
  );
}
