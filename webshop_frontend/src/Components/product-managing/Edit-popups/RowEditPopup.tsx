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
};

export default function RowEditPopup(props: RowEditPopupProps) {
  return (
    <div className="popup-grey-zone">
      <div className="popup-box">
        <form className="popup-form">
          {props.image ? (
            <>
              <h2>Edit image</h2>
              <label htmlFor="image">Image:</label>
              <input
                type="file"
                id="image"
                name="image"
                accept="image/png, image/jpeg, image/webp"
              />
              <label htmlFor="alt-text">Alt-text:</label>
              <textarea
                name="alt-text"
                id="alt-text"
                cols={40}
                rows={10}
              ></textarea>
            </>
          ) : (
            <>
                rows={10}
              />
            </>
          )}

        </form>
      </div>
    </div>
  );
}
