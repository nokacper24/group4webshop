import { useState } from "react";
import { AccordionRow, AccordionRowProps } from "./AccordionRow";

export type AccordionBodyProps = {
  rows: AccordionRowProps[];
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
            key={"row" + row.id}
            title={row.title}
            id={row.id}
            editFunction={row.editFunction}
            removeFunction={row.removeFunction}
          ></AccordionRow>
        );
      })}
    </div>
  );
}
