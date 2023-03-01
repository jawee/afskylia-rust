enum ComponentType {
    Heading1,
    Heading2,
    Heading3,
    Text,
}

struct Component {
    component_type: ComponentType,
    child: Component,
}
