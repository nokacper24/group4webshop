import { NewDescription } from "./ProductPage";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT + "/";
// check if we are in production mode
if (import.meta.env.PROD) {
    baseUrl = "/";
}

export default function DescriptionsContianer(descriptions: NewDescription[]) {
    let prev_item: undefined | NewDescription = undefined as undefined | NewDescription;
    return (
        <>
            {descriptions.map((item) => {
                if (item.full_width && prev_item !== undefined) {
                    let temp = prev_item;
                    prev_item = undefined;
                    return (
                        <>
                            <DescriptionRow
                                descriptions={[temp]}
                                key={temp.component_id}
                            />
                            <DescriptionRow
                                descriptions={[item]}
                                key={item.component_id}
                            />
                        </>
                    );
                } else if (item.full_width && prev_item === undefined) {
                    return (
                        <DescriptionRow
                            descriptions={[item]}
                            key={item.component_id}
                        />);
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
            }
            )}
            {prev_item !== undefined && (
                <DescriptionRow
                    descriptions={[prev_item]}
                    key={prev_item.component_id}
                />
            )
            }
        </>
    );
}



interface DescriptionRowProps {
    descriptions: NewDescription[]
}

function DescriptionRow(props: DescriptionRowProps) {
    return (
        <section className="description-row">
            {props.descriptions.map((item) => {
                if (item.image !== null) {
                    return (
                        <ImageDescription
                            key={item.component_id}
                            description={item}
                        />
                    );
                } else if (item.text !== null) {
                    return (
                        <TextDescription
                            key={item.component_id}
                            description={item}
                        />
                    );
                }
            })}
        </section>
    )
}

interface ImageDescriptionProps {
    description: NewDescription
}

function ImageDescription(props: ImageDescriptionProps) {
    return (
        <div className="description-component image-description">
        <img
            src={baseUrl + props.description.image?.image_path}
            alt={props.description.image?.alt_text}
        />
        </div>
    )
}

interface TextDescriptionProps {
    description: NewDescription
}

function TextDescription(props: TextDescriptionProps) {
    return (
        <div className="description-component text-description">
            <h2>{props.description.text?.text_title}</h2>
            <p>{props.description.text?.paragraph}</p>
        </div>
    )
}
