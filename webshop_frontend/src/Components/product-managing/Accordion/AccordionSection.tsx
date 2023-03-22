import { useEffect, useState } from "react";
import { AccordionBody } from "./AccordionBody";
import { AccordionHeader } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { ChangeType } from "./ChangeTypes";

export type AccordionSectionProps = {
  header: {
    title: string;
    rows: {
      title: string;
      id: number;
    }[];
  };

  sectionID: number;
  registerContentChange: (id: number, change: ChangeType) => void;
};

export function AccordionSection(props: AccordionSectionProps) {
  const [rowList, setRows] = useState<AccordionRowProps[]>([]);
  useEffect(() => {
    setRows(
      props.rows.map((row) => {
        return {
          title: row.title,
          id: row.id,
          editFunction: editRow,
          removeFunction: deleteRow,
        };
      })
    );
  });

  const addRow = (title: string, header: number) => {
    if (rowList.length < 2) {
      console.log("add");
      const newId =
        rowList.length != 0 ? Math.max(...rowList.map((row) => row.id)) + 1 : 0; //TODO: This is a naive way of generating ids. There could be an open id, but this will only find the highest.
      setRows((rows) => [
        ...rows,
        {
          title: title,
          id: newId,
          editFunction: editRow,
          removeFunction: deleteRow,
        },
      ]);
    }
  };

  const deleteRow = (id: number) => {
    console.log("delete: " + id);
    setRows((rows) => rows.filter((row) => row.id !== id));
    props.registerChange(id, ChangeType.DELETE);
  };

  const editRow = (id: number) => {
    console.log("edit: " + id);
    props.registerChange(id, ChangeType.EDIT);
  };


  };

  return (
    <>
      <AccordionHeader></AccordionHeader>
      <AccordionBody></AccordionBody>
    </>
  );
}
