import { useEffect, useState } from "react";
import { AccordionHeader } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";

export default function AccordionTable() {
    const [rowList, setRows] = useState<AccordionRowProps[]>([]);

  const editAccordion = (id: number) => {
    console.log("edit");
  };

  const deleteAccordion = (id: number) => {
    console.log("delete");
  };

  let rows: AccordionRowProps[] = [
    {
      title: "Test",
      id: 1,
      editFunction: editAccordion,
      removeFunction: deleteAccordion,
    },
    {
      title: "Test2",
      id: 2,
      editFunction: editAccordion,
      removeFunction: deleteAccordion,
    },
  ];

  useEffect(() => {
    setRows(rows);
  }, []);

  return (
    <div className="accordion-table">
      <AccordionHeader rows={rowList}></AccordionHeader>
    </div>
  );
}
