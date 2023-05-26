import { LocalDescription } from "../../../../../Interfaces";
import { AccordionRow } from "./AccordionRow";

type AccordionBodyProps = {
  rows: LocalDescription[];
  collapsed: boolean;
  swapRows: () => void;
  editRow: (id: number) => void;
  deleteRow: (id: number) => void;
};

/**
 * Simple component that keeps the rows of the accordion body.
 *
 * @param props the props of the component, must be of AccordionBodyProps type
 * @returns the React component for the Accordion body
 */
export function AccordionBody(props: AccordionBodyProps) {
  return (
    <div
      className={`accordion-body ${props.collapsed ? "hidden-element" : ""}`}
    >
      {props.rows.map((row) => {
        return (
          <AccordionRow
            key={"row" + row.component_id}
            description={row}
            swapFunction={props.swapRows}
            editFunction={props.editRow}
            removeFunction={props.deleteRow}
            isTop={row.component_id === 0}
          ></AccordionRow>
        );
      })}
    </div>
  );
}
