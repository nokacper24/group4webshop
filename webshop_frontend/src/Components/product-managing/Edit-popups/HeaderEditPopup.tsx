import { RefObject, useRef } from "react";

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
export default function RowEditPopup() {
  popupRef = useRef<HTMLDivElement>(null);
  titleRef = useRef(null);

  return (
    <div className="popup-grey-zone" ref={popupRef}>
      <div className="popup-box">
        <form className="popup-form">
          <h2>Edit paragraph</h2>
          <label htmlFor="title">Title:</label>
          <input type="text" ref={titleRef} id="title" name="title" />

          <div>
            <button
              className="hero-button popup-button"
              type="button"
              onClick={() => save()}
            >
              Add
            </button>
            <button
              className="hero-button popup-button"
              type="button"
              onClick={() => hide()}
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export function showHeaderPopup(inProps: HeaderEditPopupProps) {
  props = inProps;
  if (titleRef.current) {
    titleRef.current.value = props.title ? props.title : "";
  }
  popupRef.current?.classList.add("popup-visible");
}

function save() {
  props.informationCallBack(
    titleRef.current ? titleRef.current.value : "Undefined Header"
  );
  hide();
}

function hide() {
  popupRef.current?.classList.remove("popup-visible");
}
