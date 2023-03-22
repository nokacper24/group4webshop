import { useState } from "react";
import { AccordionRowProps } from "./AccordionRow";

export function AccordionBody() {
  const [rowList, setRows] = useState<AccordionRowProps[]>([]);
export type AccordionBodyProps = {
  rows: AccordionRowProps[];
};

  return (
    <>
      {rowList.map((row) => {
        return <AccordionRowProps></AccordionRowProps>;
      })}
    </>
  );
}
