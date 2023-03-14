import { useEffect, useState } from "react";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
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
    addChange(id, ChangeType.EDIT);
  };

  const deleteRow = (id: number) => {
    console.log("delete: " + id);
    setRows((rows) => rows.filter((row) => row.id !== id));
    addChange(id, ChangeType.DELETE);
  };

  const moveRows = (id1: number, id2: number) => {
    console.log("move: " + id1 + " to " + id2);

    setRows((rows) => {
      const index1 = rows.findIndex((row) => row.id === id1);
      const index2 = rows.findIndex((row) => row.id === id2);
      const temp = rows[index1];
      rows[index1] = rows[index2];
      rows[index2] = temp;
      return rows;
    });

    addChange(id1, ChangeType.MOVE);
    addChange(id2, ChangeType.MOVE);
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

  const [headerList, setHeaders] = useState<AccordionHeaderProps[]>([]);

  const addHeader = (title: string) => {
    const index = headerList.length;
    setHeaders((headers) => [
      ...headers,
      {
        title: title,
        index: index,
        addRow: addRow,
        rows: rowList,
      },
    ]);
  };

  return (
    <div className="accordion-table">
      <AccordionHeader
      index={0}
        title={"Hello"}
        addRow={addRow}
        rows={rowList}
      ></AccordionHeader>
    </div>
  );
}
