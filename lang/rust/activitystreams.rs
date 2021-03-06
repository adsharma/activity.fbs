// automatically generated by the FlatBuffers compiler, do not modify
#[derive(PartialEq,Clone)]
pub struct Date {
    pub ts: Option<u64>,
}

#[derive(PartialEq,Clone)]
pub struct Duration {
    pub duration: Option<u64>,
}

#[derive(PartialEq,Clone)]
pub struct Language {
    pub code: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct URL {
    pub url: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct RelativeLink {
    pub link: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct MimeType {
    pub encoding: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct Bool {
    pub val: Option<bool>,
}

#[derive(PartialEq,Clone)]
pub struct String {
    pub val: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct IntransitiveActivity {
}

#[derive(PartialEq,Clone)]
pub struct Activity {
    pub actor: ObjectOrLink,
    pub instrument: ObjectOrLink,
    pub origin: ObjectOrLink,
    pub result: ObjectOrLink,
    pub target: ObjectOrLink,
}

#[derive(PartialEq,Clone)]
pub struct Object {
    pub attachment: ObjectOrLink,
    pub attributedTo: ObjectOrLink,
    pub audience: ObjectOrLink,
    pub bcc: ObjectOrLink,
    pub bto: ObjectOrLink,
    pub cc: ObjectOrLink,
    pub context: ObjectOrLink,
    pub generator: ObjectOrLink,
    pub icon: ImageOrLink,
    pub image: ImageOrLink,
    pub inReplyTo: ObjectOrLink,
    pub location: ObjectOrLink,
    pub object: ObjectOrLink,
    pub preview: ObjectOrLink,
    pub replies: Collection,
    pub tag: ObjectOrLink,
    pub to: ObjectOrLink,
    pub url: URLOrLink,
    pub altitude: Option<f32>,
    pub content: Option<String>,
    pub name: Option<String>,
    pub duration: Duration,
    pub mediaType: MimeType,
    pub endTime: Date,
    pub published: Date,
    pub source_property: ObjectOrLink,
    pub startTime: Date,
    pub summary: Option<String>,
    pub updated: Date,
    pub likes: MaybeOrderedCollection,
    pub shares: MaybeOrderedCollection,
}

#[derive(PartialEq,Clone)]
pub struct Link {
    pub attributedTo: ObjectOrLink,
    pub preview: ObjectOrLink,
    pub name: Option<String>,
    pub height: Option<u32>,
    pub href: URL,
    pub hreflang: Language,
    pub mediaType: MimeType,
    pub rel: RelativeLink,
    pub summary: Option<String>,
    pub width: Option<u32>,
}

#[derive(PartialEq,Clone)]
pub struct Collection {
    pub current: PageOrLink,
    pub first: PageOrLink,
    pub last: PageOrLink,
    pub items: ObjectOrLink,
    pub totalItems: Option<u32>,
}

#[derive(PartialEq,Clone)]
pub struct OrderedCollection {
    pub orderedItems: ObjectOrLink,
}

#[derive(PartialEq,Clone)]
pub struct Question {
    pub oneOf: ObjectOrLink,
    pub anyOf: ObjectOrLink,
    pub closed: Various,
}

#[derive(PartialEq,Clone)]
pub struct CollectionPage {
    pub next: PageOrLink,
    pub prev: PageOrLink,
    pub partOf: CollectionOrLink,
}

#[derive(PartialEq,Clone)]
pub struct Place {
    pub accuracy: Option<f32>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub radius: Option<f32>,
    pub units: StringOrURL,
}

#[derive(PartialEq,Clone)]
pub struct Image {
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(PartialEq,Clone)]
pub struct OrderedCollectionPage {
    pub startIndex: Option<u32>,
}

#[derive(PartialEq,Clone)]
pub struct Relationship {
    pub subject: ObjectOrLink,
    pub relationship: Object,
}

#[derive(PartialEq,Clone)]
pub struct Profile {
    pub describes: Object,
}

#[derive(PartialEq,Clone)]
pub struct Tombstone {
    pub formerType: ObjectOrString,
    pub deleted: Date,
}

#[derive(PartialEq,Clone)]
pub struct Application {
    pub inbox: OrderedCollection,
    pub outbox: OrderedCollection,
    pub following: MaybeOrderedCollection,
    pub followers: MaybeOrderedCollection,
    pub liked: MaybeOrderedCollection,
    pub streams: MaybeOrderedCollection,
    pub preferredUsername: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct Group {
    pub inbox: OrderedCollection,
    pub outbox: OrderedCollection,
    pub following: MaybeOrderedCollection,
    pub followers: MaybeOrderedCollection,
    pub liked: MaybeOrderedCollection,
    pub streams: MaybeOrderedCollection,
    pub preferredUsername: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct Organization {
    pub inbox: OrderedCollection,
    pub outbox: OrderedCollection,
    pub following: MaybeOrderedCollection,
    pub followers: MaybeOrderedCollection,
    pub liked: MaybeOrderedCollection,
    pub streams: MaybeOrderedCollection,
    pub preferredUsername: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct Person {
    pub inbox: OrderedCollection,
    pub outbox: OrderedCollection,
    pub following: MaybeOrderedCollection,
    pub followers: MaybeOrderedCollection,
    pub liked: MaybeOrderedCollection,
    pub streams: MaybeOrderedCollection,
    pub preferredUsername: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct Service {
    pub inbox: OrderedCollection,
    pub outbox: OrderedCollection,
    pub following: MaybeOrderedCollection,
    pub followers: MaybeOrderedCollection,
    pub liked: MaybeOrderedCollection,
    pub streams: MaybeOrderedCollection,
    pub preferredUsername: Option<String>,
}

#[derive(PartialEq,Clone)]
pub struct Accept {
}

#[derive(PartialEq,Clone)]
pub struct TentativeAccept {
}

#[derive(PartialEq,Clone)]
pub struct Add {
}

#[derive(PartialEq,Clone)]
pub struct Arrive {
}

#[derive(PartialEq,Clone)]
pub struct Create {
}

#[derive(PartialEq,Clone)]
pub struct Delete {
}

#[derive(PartialEq,Clone)]
pub struct Follow {
}

#[derive(PartialEq,Clone)]
pub struct Ignore {
}

#[derive(PartialEq,Clone)]
pub struct Join {
}

#[derive(PartialEq,Clone)]
pub struct Leave {
}

#[derive(PartialEq,Clone)]
pub struct Like {
}

#[derive(PartialEq,Clone)]
pub struct Offer {
}

#[derive(PartialEq,Clone)]
pub struct Invite {
}

#[derive(PartialEq,Clone)]
pub struct Reject {
}

#[derive(PartialEq,Clone)]
pub struct TentativeReject {
}

#[derive(PartialEq,Clone)]
pub struct Remove {
}

#[derive(PartialEq,Clone)]
pub struct Undo {
}

#[derive(PartialEq,Clone)]
pub struct Update {
}

#[derive(PartialEq,Clone)]
pub struct View {
}

#[derive(PartialEq,Clone)]
pub struct Listen {
}

#[derive(PartialEq,Clone)]
pub struct Read {
}

#[derive(PartialEq,Clone)]
pub struct Move {
}

#[derive(PartialEq,Clone)]
pub struct Travel {
}

#[derive(PartialEq,Clone)]
pub struct Announce {
}

#[derive(PartialEq,Clone)]
pub struct Block {
}

#[derive(PartialEq,Clone)]
pub struct Flag {
}

#[derive(PartialEq,Clone)]
pub struct Dislike {
}

#[derive(PartialEq,Clone)]
pub struct Question {
}

#[derive(PartialEq,Clone)]
enum PageOrLink {
    CollectionPage,
    Link,
}

#[derive(PartialEq,Clone)]
enum ImageOrLink {
    Image,
    Link,
}

#[derive(PartialEq,Clone)]
enum CollectionOrLink {
    Link,
    Collection,
}

#[derive(PartialEq,Clone)]
enum ObjectOrLink {
    Link,
    Object,
}

#[derive(PartialEq,Clone)]
enum Various {
    Object,
    Link,
    Date,
    Bool,
}

#[derive(PartialEq,Clone)]
enum ObjectOrString {
    Object,
    String,
}

#[derive(PartialEq,Clone)]
enum MaybeOrderedCollection {
    OrderedCollection,
    Collection,
}

#[derive(PartialEq,Clone)]
enum URLOrLink {
    URL,
    Link,
}

#[derive(PartialEq,Clone)]
enum StringOrURL {
    String,
    URL,
}

#[derive(PartialEq,Clone)]
enum ActivityStream {
    Accept,
    TentativeAccept,
    Add,
    Arrive,
    Create,
    Delete,
    Follow,
    Ignore,
    Join,
    Leave,
    Like,
    Offer,
    Invite,
    Reject,
    TentativeReject,
    Remove,
    Undo,
    Update,
    View,
    Listen,
    Read,
    Move,
    Travel,
    Announce,
    Block,
    Flag,
    Dislike,
    Question,
}

#[derive(PartialEq,Clone)]
enum Actor {
    Application,
    Group,
    Organization,
    Person,
    Service,
}
