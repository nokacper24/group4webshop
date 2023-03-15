import { useState } from "react";
import { AccordionRowProps } from "./AccordionRow";

export function AccordionBody() {
  const [rowList, setRows] = useState<AccordionRowProps[]>([]);

  return (
    <>
      {rowList.map((row) => {
        return <AccordionRowProps></AccordionRowProps>;
      })}
    </>
  );
}
