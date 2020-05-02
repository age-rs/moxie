//! Traits which define attributes available to subsets of elements.

use crate::{
    elements::{
        embedding::*, forms::*, interactive::*, media::*, metadata::*, scripting::*, table::*,
        text_content::*, text_semantics::*,
    },
    prelude::*,
};

attr_trait! {
    /// List of types the server accepts, typically a file type.
    AcceptAttr::accept for
    Form, Input
}

attr_trait! {
    /// Specifies the horizontal alignment of the element.
    AlignAttr::align for
    TableCaption, TableColumn, TableColumnGroup, HorizontalRule, InlineFrame, Image, Table,
    TableBody, TableCell, TableFooter, TableHeaderCell, TableHeader, TableRow
}

attr_trait! {
    /// Alternative text in case an image can't be displayed.
    AltAttr::alt for
    Area, Image, Input
}

attr_trait! {
    /// Indicates whether controls in this form can by default have their values automatically
    /// completed by the browser.
    AutocompleteAttr::autocomplete for
    Form, Input, Select, TextArea
}

attr_trait! {
    /// The element should be automatically focused after the page loaded.
    AutofocusAttr::autofocus for
    Button, Input, Select, TextArea
}

attr_trait! {
    /// The audio or video should play as soon as possible.
    AutoplayAttr::autoplay for
    Audio, Video
}

attr_trait! {
    /// Contains the time range of already buffered media.
    BufferedAttr::buffered for
    Audio, Video
}

attr_trait! {
    /// Contains a URI which points to the source of the quote or change.
    CiteAttr::cite for
    BlockQuote, Deleted, Inserted, Quotation
}

attr_trait! {
    /// The colspan attribute defines the number of columns a cell should span.
    ColSpanAttr::colspan for
    TableCell, TableHeaderCell
}

attr_trait! {
    /// Indicates whether the browser should show playback controls to the user.
    ControlsAttr::controls for
    Audio, Video
}

attr_trait! {
    /// How the element handles cross-origin requests.
    CrossOriginAttr::crossorigin for
    Audio, Image, ExternalResourceLink, Script, Video
}

attr_trait! {
    /// Indicates the date and time associated with the element.
    DateTimeAttr::datetime for
    Deleted, Inserted, Time
}

attr_trait! {
    /// The directionality of the element.
    DirNameAttr::dirname for
    Input, TextArea
}

attr_trait! {
    /// Indicates whether the user can interact with the element.
    DisabledAttr::disabled for
    Button, FieldSet, Input, OptionGroup, OptionItem, Select, TextArea
}

attr_trait! {
    /// Indicates that the hyperlink is to be used for downloading a resource.
    DownloadAttr::download for
    Anchor, Area
}

attr_trait! {
    /// Describes elements which belongs to this one.
    ForAttr::for_ for
    Label, Output
}

attr_trait! {
    /// Indicates the form that is the owner of the element.
    FormAttr::form for
    Button, FieldSet, Input, Label, Meter, Object, Output, Progress, Select, TextArea
}

attr_trait! {
    /// Indicates the action of the element, overriding the action defined in the Form.
    FormActionAttr::formaction for
    Input, Button
}

attr_trait! {
    /// If the button/input is a submit button (type="submit"), this attribute sets the encoding
    /// type to use during form submission. If this attribute is specified, it overrides the enctype
    /// attribute of the button's form owner.
    FormEncTypeAttr::formenctype for
    Button, Input
}

attr_trait! {
    /// If the button/input is a submit button (type="submit"), this attribute sets the submission
    /// method to use during form submission (GET, POST, etc.). If this attribute is specified, it
    /// overrides the method attribute of the button's form owner.
    FormMethodAttr::formmethod for
    Button, Input
}

attr_trait! {
    /// If the button/input is a submit button (type="submit"), this boolean attribute specifies
    /// that the form is not to be validated when it is submitted. If this attribute is specified,
    /// it overrides the novalidate attribute of the button's form owner.
    FormNoValidateAttr::formnovalidate for
    Button, Input
}

attr_trait! {
    /// If the button/input is a submit button (type="submit"), this attribute specifies the
    /// browsing context (for example, tab, window, or inline frame) in which to display the
    /// response that is received after submitting the form. If this attribute is specified, it
    /// overrides the target attribute of the button's form owner.
    FormTargetAttr::formtarget for
    Button, Input
}

attr_trait! {
    /// IDs of the TableHeaderCell elements which applies to this element.
    HeadersAttr::headers for
    TableCell, TableHeaderCell
}

attr_trait! {
    /// The element's height attribute.
    HeightAttr::height for
    Canvas, Embed, InlineFrame, Image, Input, Object, Video
}

attr_trait! {
    /// The URL of a linked resource.
    HrefAttr::href for
    Anchor, Area, Base, ExternalResourceLink
}

attr_trait! {
    /// Specifies the language of the linked resource.
    HrefLangAttr::hreflang for
    Anchor, Area, ExternalResourceLink
}

attr_trait! {
    /// Indicates the relative fetch priority for the resource.
    ImportanceAttr::importance  for
    InlineFrame, Image, ExternalResourceLink, Script
}

attr_trait! {
    /// Specifies a Subresource Integrity value that allows browsers to verify what they fetch.
    IntegrityAttr::integrity for
    ExternalResourceLink, Script
}

attr_trait! {
    /// Specifies a user-readable title of the element.
    LabelAttr::label for
    OptionGroup, OptionItem, Track
}

attr_trait! {
    /// Indicates whether the media should start playing from the start when it's finished.
    LoopAttr::loop_ for
    Audio, Video
}

attr_trait! {
    /// Indicates the maximum value allowed.
    MaxAttr::max for
    Input, Meter, Progress
}

attr_trait! {
    /// Defines the maximum number of characters allowed in the element.
    MaxLengthAttr::maxlength for
    Input, TextArea
}

attr_trait! {
    /// Defines the minimum number of characters allowed in the element.
    MinLengthAttr::minlength for
    Input, TextArea
}

attr_trait! {
    /// Specifies a hint of the media for which the linked resource was designed.
    MediaAttr::media for
    Anchor, Area, ExternalResourceLink, Source, Style
}

attr_trait! {
    /// Indicates the minimum value allowed.
    MinAttr::min for
    Input, Meter
}

attr_trait! {
    /// Indicates whether multiple values can be entered in an input of the type email or file.
    MultipleAttr::multiple for
    Input, Select
}

attr_trait! {
    /// Indicates whether the audio will be initially silenced on page load.
    MutedAttr::muted for
    Audio, Video
}

attr_trait! {
    /// Name of the element. For example used by the server to identify the fields in form submits.
    NameAttr::name for
    Button, Form, FieldSet, InlineFrame, Input, Object, Output, Select, TextArea, Map, Meta, Param
}

attr_trait! {
    /// The ping attribute specifies a space-separated list of URLs to be notified if a user follows the hyperlink.
    PingAttr::ping for
    Anchor, Area
}

attr_trait! {
    /// Provides a hint to the user of what can be entered in the field.
    PlaceholderAttr::placeholder for
    Input, TextArea
}

attr_trait! {
    /// Indicates whether the whole resource, parts of it or nothing should be preloaded.
    PreloadAttr::preload for
    Audio, Video
}

attr_trait! {
    /// Indicates whether the element can be edited.
    ReadOnlyAttr::readonly for
    Input, TextArea
}

attr_trait! {
    /// Specifies which referrer is sent when fetching the resource.
    ReferrerPolicyAttr::referrerpolicy for
    Anchor, Area, InlineFrame, Image, ExternalResourceLink, Script
}

attr_trait! {
    /// Specifies the relationship of the target object to the link object.
    RelAttr::rel for
    Anchor, Area, ExternalResourceLink
}

attr_trait! {
    /// Indicates whether this element is required to fill out or not.
    RequiredAttr::required for
    Input, Select, TextArea
}

attr_trait! {
    /// Defines the number of rows a table cell should span over.
    RowSpanAttr::rowspan for
    TableCell, TableHeaderCell
}

attr_trait! {
    /// Defines the width of the element (in pixels). If the element's type attribute is text or
    /// password then it's the number of characters.
    SizeAttr::size for
    Input, Select
}

attr_trait! {
    /// One or more strings separated by commas, indicating a set of source sizes.
    SizesAttr::sizes for
    ExternalResourceLink, Image, Source
}

attr_trait! {
    /// This attribute contains a positive integer indicating the number of consecutive columns the
    /// `<col>` element spans. If not present, its default value is 1.
    SpanAttr::span for
    TableColumn, TableColumnGroup
}

attr_trait! {
    /// The URL of the embeddable content.
    SrcAttr::src for
    Audio, Embed, InlineFrame, Image, Input, Script, Source, Track, Video
}

attr_trait! {
    /// One or more responsive image candidates.
    SrcSetAttr::srcset for
    Image, Source
}

attr_trait! {
    /// Specifies where to display a resource.
    TargetAttr::target for
    Anchor, Area, Base, Form
}

attr_trait! {
    /// Defines the type of the element.
    TypeAttr::type_ for
    Button, Input, Embed, Object, Script, Source, Style, Menu
}

attr_trait! {
    /// A hash-name reference to a `<map>` element; that is a '#' followed by the value of a name of
    /// a map element.
    UsemapAttr::usemap for
    Image, Object
}

attr_trait! {
    /// Defines a default value which will be displayed in the element on page load.
    ValueAttr::value for
    Button, Data, Input, ListItem, Meter, OptionItem, Progress, Param
}

attr_trait! {
    /// For the elements listed here, this establishes the element's width.
    WidthAttr::width for
    Canvas, Embed, InlineFrame, Image, Input, Object, Video
}