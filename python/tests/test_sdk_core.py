from memory_sdk.client import MemoryClient


def test_python_sdk_upsert_and_recall(tmp_path):
    client = MemoryClient(str(tmp_path / "memory.db"))
    client.upsert_fact(
        namespace="workspace",
        scope_id="localmemos",
        entity="project",
        attribute="preferred_package_manager",
        value="npm",
        source_kind="manual",
    )
    client.upsert_fact(
        namespace="workspace",
        scope_id="localmemos",
        entity="project",
        attribute="preferred_package_manager",
        value="bun",
        source_kind="manual",
    )
    result = client.recall(
        namespace="workspace",
        scope_id="localmemos",
        entity="project",
        attribute="preferred_package_manager",
    )
    assert result["facts"][0]["value_text"] == "bun"
    history = client.history(
        namespace="workspace",
        scope_id="localmemos",
        entity="project",
        attribute="preferred_package_manager",
    )
    assert len(history) == 2
