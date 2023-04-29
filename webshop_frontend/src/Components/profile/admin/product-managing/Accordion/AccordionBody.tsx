import { SimpleDescription } from "../../../../../Interfaces";
import { AccordionRow } from "./AccordionRow";

type AccordionBodyProps = {
  rows: SimpleDescription[];
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
    <div className="accordion-body">
      {props.rows.map((row) => {
        return (
          <AccordionRow
            key={"row" + row.component_id}
            description={row}
            editFunction={props.editRow}
            removeFunction={props.deleteRow}
          ></AccordionRow>
        );
      })}
    </div>
  );
}
