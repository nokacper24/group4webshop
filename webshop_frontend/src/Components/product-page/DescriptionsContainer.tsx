import { NewDescription } from "./ProductPage";


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
        <p>Description Row with number of items: {props.descriptions.length}</p>
    )
}