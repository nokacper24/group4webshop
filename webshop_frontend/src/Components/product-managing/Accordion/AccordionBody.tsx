import { useState } from "react";
import { AccordionRow, AccordionRowProps } from "./AccordionRow";

export type AccordionBodyProps = {
  rows: AccordionRowProps[];
};

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
