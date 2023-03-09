import { useEffect, useState } from "react";
import { AccordionHeader } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { ChangeType } from "./ChangeTypes";

export default function AccordionTable() {
  const [changes, setChanges] = useState<Map<string, number[]>>(new Map());

  useEffect(() => {
    // Sets up the map so that it can register changes
    setChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(type, []);
      }
      return changes;
    });
  });

  const editRow = (id: number) => {
    console.log("edit: " + id);
  };

  const deleteRow = (id: number) => {
    console.log("delete: " + id);
    setRows((rows) => rows.filter((row) => row.id !== id));
  };

  const addChange = (id: number, change: ChangeType) => {
    if (!changes.get(change)?.includes(id)) {
      changes.get(change)?.push(id);
    }
    if (change === ChangeType.DELETE) {
      changes.get(ChangeType.EDIT)?.filter((changeId) => changeId !== id);
      changes.get(ChangeType.MOVE)?.filter((changeId) => changeId !== id);
    }
  };

  const rows: AccordionRowProps[] = [
    {
      title: "Test",
      id: 1,
      editFunction: editRow,
      removeFunction: deleteRow,
    },
    {
      title: "Test2",
      id: 2,
      editFunction: editRow,
      removeFunction: deleteRow,
    },
  ];

  const [rowList, setRows] = useState<AccordionRowProps[]>(rows);

  return (
    <div className="accordion-table">
      <AccordionHeader rows={rowList}></AccordionHeader>
    </div>
  );
}
