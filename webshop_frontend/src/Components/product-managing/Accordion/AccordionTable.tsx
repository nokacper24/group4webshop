import { useEffect, useState } from "react";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { AccordionSection, AccordionSectionProps } from "./AccordionSection";
import { ChangeType } from "./ChangeTypes";
import { showPopup } from "../Edit-popups/RowEditPopup";

/**
 * Renders a table consisting of the header and a body with rows connected to the header through
 * this component.
 *
 * @returns The React component for the Accordion table
 */
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

  /**
   * Registers changes to the content of the table. This is used to keep track of what has changed and what needs to be saved.
   *
   * @param id The ID of the section that has changed
   * @param change The type of change that has been made
   */
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

  /**
   * Deletes a section, including its header and rows in the body, from the table.
   * Makes react re-render.
   *
   * @param id The ID of the section that has changed
   */
  const deleteSection = (id: number) => {
    const index = sectionList.findIndex((section) => section.sectionID === id);
    /* delete sectionList[index];
    setSectionList([ ...sectionList ]);
    console.log(sectionList); */

    const newSections = sectionList.filter(
      (section) => section.sectionID !== id
    );
    console.log(newSections);
    setSectionList(newSections);
  };

  const [sectionList, setSectionList] = useState<AccordionSectionProps[]>([
    {
      header: {
        title: "Test",
      },
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
      sectionID: 0,
      registerContentChange: registerContentChange,
      deleteSection: deleteSection,
    },
  ]);

  /**
   * Creates a new section in the table.
   * Makes react re-render.
   *
   * @param title The title of the new section
   */
  const newSection = (title: string) => {
    const id = sectionList.length;
    sectionList.push({
      header: {
        title: title,
      },
      rows: [],
      sectionID: id,
      registerContentChange: registerContentChange,
      deleteSection: deleteSection,
    });
    setSectionList([...sectionList]);
    showPopup();
    console.log(sectionList);

    /* setSectionList((sections) => [
      ...sections,
      {
        header: {
          title: title,
          rows: [],
        },
        sectionID: id,
        registerContentChange: registerContentChange,
        deleteSection: deleteSection,
      },
    ]);
    console.log(sectionList); */
  };

  return (
    <div className="accordion-table">
      <button onClick={() => newSection("Testy")}>New section</button>
      {sectionList.map((section) => {
        return (
          <AccordionSection
            key={section.sectionID}
            header={section.header}
            rows={section.rows}
            sectionID={section.sectionID}
            registerContentChange={registerContentChange}
            deleteSection={deleteSection}
          ></AccordionSection>
        );
      })}
    </div>
  );
}
