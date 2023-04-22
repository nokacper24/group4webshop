import { RefObject, useRef } from "react";

/**
 * Props for the RowEditPopup component.
 * Image indicates if the content should be interpreted as an image or not.
 * The title and content can be undefined if a new row is being created.
 * If the content is an image, the title is the alt text.
 */
export type RowEditPopupProps = {
  image: boolean;
  title: string | undefined;
  content: string | undefined;
  informationCallBack: (image: boolean, title: string, content: string) => void;
};

let popupRef: RefObject<HTMLDivElement>;
let imageRef: RefObject<HTMLInputElement>;
let titleRef: RefObject<HTMLInputElement>;
let altTextRef: RefObject<HTMLTextAreaElement>;
let contentRef: RefObject<HTMLTextAreaElement>;
let props: RowEditPopupProps = {
  image: false,
  title: undefined,
  content: undefined,
  informationCallBack: () => {},
};
export default function RowEditPopup() {
  popupRef = useRef(null);
  imageRef = useRef(null);
  titleRef = useRef(null);
  contentRef = useRef(null);

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  return (
    <div className="popup-grey-zone" ref={popupRef}>
      <div className="popup-box">
        <form
          className="popup-form"
          onSubmit={(event) => {
            handleSubmit(event);
          }}
        >
          {props.image ? (
            <>
              <h2>Edit image</h2>
              <label htmlFor="image">Image:</label>
              <input
                type="file"
                id="image"
                name="image"
                accept="image/png, image/jpeg, image/webp"
                ref={imageRef}
                defaultValue={props.content ? props.content : ""}
              />
              <label htmlFor="alt-text">Alt-text:</label>
              <textarea
                name="alt-text"
                id="alt-text"
                cols={40}
                rows={10}
                ref={altTextRef}
                defaultValue={props.title ? props.title : ""}
              ></textarea>
            </>
          ) : (
            <>
              <h2>Edit paragraph</h2>
              <label htmlFor="title">Title:</label>
              <input
                type="text"
                id="title"
                name="title"
                defaultValue={props.title ? props.title : ""}
                ref={titleRef}
              />
              <label htmlFor="paragraph">Paragraph:</label>
              <textarea
                id="paragraph"
                defaultValue={props.content ? props.content : ""}
                cols={40}
                rows={10}
                ref={contentRef}
              />
            </>
          )}

          <div>
            <button
              className="hero-button popup-button"
              type="button"
              onClick={() => save()}
            >
              Save
            </button>
            <button
              className="hero-button popup-button"
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

export function showPopup(inProps: RowEditPopupProps) {
  props = inProps;
  popupRef.current?.classList.add("popup-visible");
  imageRef.current
    ? (imageRef.current.value = props.content ? props.content : "")
    : undefined; //TODO: Implement "Something has gone wrong"
  titleRef.current
    ? (titleRef.current.value = props.title ? props.title : "")
    : undefined; //TODO: Implement "Something has gone wrong"
  contentRef.current
    ? (contentRef.current.value = props.content ? props.content : "")
    : undefined; //TODO: Implement "Something has gone wrong"
}

function save() {
  props.informationCallBack(
    props.image,
    titleRef.current?.value ? titleRef.current.value : "",
    contentRef.current?.value ? contentRef.current.value : ""
  );
  hidePopup();
}

function hidePopup() {
  popupRef.current?.classList.remove("popup-visible");
}
