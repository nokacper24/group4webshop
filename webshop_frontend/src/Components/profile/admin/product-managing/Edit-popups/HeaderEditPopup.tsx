import { RefObject, useRef } from "react";

//TODO: Update to follow the best practices. This is currently an old copy of the RowEditPopup component with minor changes.

/**
 * Props for the HeaderEditPopup component.
 * The title can be undefined if a new header is being created.
 * The informationCallBack is the function that should be called with the information from the form.
 */
export type HeaderEditPopupProps = {
  title: string | undefined;
  informationCallBack: (title: string) => void;
};

let popupRef: RefObject<HTMLDivElement>;
let titleRef: RefObject<HTMLInputElement>;
let props: HeaderEditPopupProps = {
  title: undefined,
  informationCallBack: () => {},
};
export default function HeaderEditPopup() {
  popupRef = useRef<HTMLDivElement>(null);
  titleRef = useRef(null);

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  return (
    <div className="popup-grey-zone" ref={popupRef}>
      <div>
        <form
          className="form-container container"
          method="POST"
          onSubmit={(event) => {
            handleSubmit(event);
          }}
        >
          <h2>Edit section header</h2>
          <label htmlFor="title">Title:</label>
          <input type="text" ref={titleRef} id="title" name="title" />

          <div className="button-container popup-button-container">
            <button
              className="default-button small-button popup-button"
              type="button"
              onClick={() => save()}
            >
              Add
            </button>
            <button
              className="default-button small-button popup-button"
              type="button"
              onClick={() => hidePopup()}
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

/**
 * Shows the popup with the given props.
 * 
 * @param inProps 
 */
export function showHeaderPopup(inProps: HeaderEditPopupProps) {
  props = inProps;
  if (titleRef.current) {
    titleRef.current.value = props.title ? props.title : "";
  }
  popupRef.current?.classList.add("popup-visible");
}

/**
 * Saves the information from the form and calls the informationCallBack function.
 */
function save() {
  props.informationCallBack(
    titleRef.current ? titleRef.current.value : "Undefined Header"
  );
  hidePopup();
}

/**
 * Hides the popup.
 */
function hidePopup() {
  popupRef.current?.classList.remove("popup-visible");
}
