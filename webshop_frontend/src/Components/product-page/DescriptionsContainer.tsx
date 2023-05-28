import { Description } from "../../Interfaces";

export default function DescriptionsContainer(descriptions: Description[]) {
  let prev_item: undefined | Description = undefined as undefined | Description;
  return (
    <>
      {descriptions.map((item) => {
        if (item.full_width && prev_item !== undefined) {
          let temp = prev_item;
          prev_item = undefined;
          return (
            <>
              <DescriptionRow descriptions={[temp]} key={temp.component_id} />
              <DescriptionRow descriptions={[item]} key={item.component_id} />
            </>
          );
        } else if (item.full_width && prev_item === undefined) {
          return (
            <DescriptionRow descriptions={[item]} key={item.component_id} />
          );
        } else if (!item.full_width && prev_item !== undefined) {
          let temp = prev_item;
          prev_item = undefined;
          return (
            <DescriptionRow
              descriptions={[temp, item]}
              key={temp.component_id}
            />
          );
        } else if (!item.full_width && prev_item === undefined) {
          prev_item = item;
        }
      })}
      {prev_item !== undefined && (
        <DescriptionRow
          descriptions={[prev_item]}
          key={prev_item.component_id}
        />
      )}
    </>
  );
}

interface DescriptionRowProps {
  descriptions: Description[];
}

function DescriptionRow(props: DescriptionRowProps) {
  if (props.descriptions.length === 1) {
    // if exactly one item in row, must be full width
    props.descriptions[0].full_width = true;
  }
  return (
    <section className="description-row">
      {props.descriptions.map((item) => {
        if (item.image !== null) {
          return (
            <ImageDescription key={item.component_id} description={item} />
          );
        } else if (item.text !== null) {
          return <TextDescription key={item.component_id} description={item} />;
        }
      })}
    </section>
  );
}

interface ImageDescriptionProps {
  description: Description;
}

function ImageDescription(props: ImageDescriptionProps) {
  let classes: string;
  if (props.description.full_width) {
    classes = "image-description full-width-desc";
  } else {
    classes = "description-component image-description";
  }

  return (
    <div className={classes}>
      <img
        src={props.description.image?.image_path}
        alt={props.description.image?.alt_text}
      />
    </div>
  );
}

interface TextDescriptionProps {
  description: Description;
}

function TextDescription(props: TextDescriptionProps) {
  let classes: string;
  if (props.description.full_width) {
    classes = "text-description full-width-desc";
  } else {
    classes = "description-component text-description";
  }

  return (
    <div className={classes}>
      <h2>{props.description.text?.text_title}</h2>
      <p>{props.description.text?.paragraph}</p>
    </div>
  );
}
