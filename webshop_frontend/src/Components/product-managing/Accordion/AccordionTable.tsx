import { useEffect, useState } from "react";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { AccordionSection, AccordionSectionProps } from "./AccordionSection";
import { ChangeType } from "./ChangeTypes";

export default function AccordionTable() {
  const [priorityChanges, setPriorityChanges] = useState<Map<number, number>>(
    new Map()
  ); //ID as key, priority as value
  const [contentChanges, setContentChanges] = useState<
    Map<ChangeType, number[]>
  >(new Map()); //The type of change as key, list of IDs that has had that change as value

  useEffect(() => {
    // Sets up the map so that it can register changes
    setContentChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(ChangeType[type as keyof typeof ChangeType], []);
      }
      return changes;
    });
  });

  const registerContentChange = (id: number, change: ChangeType) => {
    if (!contentChanges.get(change)?.includes(id)) {
      contentChanges.get(change)?.push(id);
    }
    if (change === ChangeType.Delete) {
      contentChanges
        .get(ChangeType.Edit)
        ?.filter((changeId) => changeId !== id);
      priorityChanges.delete(id);
    }
  };

  const [sectionList, setSectionList] = useState<AccordionSectionProps[]>([
    {
      header: {
        title: "Test",
        rows: [
          {
            title: "Test",
            id: 1,
          },
          {
            title: "Test2",
            id: 2,
          },
        ],
      },
      sectionID: 0,
      registerContentChange: registerContentChange,
      deleteSection: deleteSection,
    },

  const newSection = (title: string) => {
    const id = sectionList.length;
    setSectionList((sections) => [
      ...sections,
      {
        header: {
          title: title,
          rows: [],
        },
        sectionID: id,
        registerContentChange: registerContentChange,
      },
    ]);
  };

  return (
    <div className="accordion-table">
      <button onClick={() => newSection("Testy")}>New section</button>
      {sectionList.map((section) => {
        return (
          <AccordionSection
            key={section.sectionID}
            header={section.header}
            sectionID={section.sectionID}
            registerContentChange={section.registerContentChange}
          ></AccordionSection>
        );
      })}
    </div>
  );
}
