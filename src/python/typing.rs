use lazy_static::lazy_static;
use std::collections::BTreeSet;

lazy_static! {
    static ref ANNOTATED_SUBSCRIPTS: BTreeSet<&'static str> = BTreeSet::from([
        "AbstractAsyncContextManager",
        "AbstractContextManager",
        "AbstractSet",
        "AsyncContextManager",
        "AsyncGenerator",
        "AsyncIterable",
        "AsyncIterator",
        "Awaitable",
        "BinaryIO",
        "BsdDbShelf",
        "ByteString",
        "Callable",
        "ChainMap",
        "ClassVar",
        "Collection",
        "Concatenate",
        "Container",
        "ContextManager",
        "Coroutine",
        "Counter",
        "Counter",
        "DbfilenameShelf",
        "DefaultDict",
        "Deque",
        "Dict",
        "Field",
        "Final",
        "FrozenSet",
        "Generator",
        "Iterator",
        "Generic",
        "IO",
        "ItemsView",
        "Iterable",
        "Iterator",
        "KeysView",
        "LifoQueue",
        "List",
        "Mapping",
        "MappingProxyType",
        "MappingView",
        "Match",
        "MutableMapping",
        "MutableSequence",
        "MutableSet",
        "Optional",
        "OrderedDict",
        "PathLike",
        "Pattern",
        "PriorityQueue",
        "Protocol",
        "Queue",
        "Reversible",
        "Sequence",
        "Set",
        "Shelf",
        "SimpleQueue",
        "TextIO",
        "Tuple",
        "Type",
        "TypeGuard",
        "Union",
        "ValuesView",
        "WeakKeyDictionary",
        "WeakMethod",
        "WeakSet",
        "WeakValueDictionary",
        "cached_property",
        "defaultdict",
        "deque",
        "dict",
        "frozenset",
        "list",
        "partialmethod",
        "set",
        "tuple",
        "type",
    ]);
}

pub fn is_annotated_subscript(name: &str) -> bool {
    ANNOTATED_SUBSCRIPTS.contains(name)
}