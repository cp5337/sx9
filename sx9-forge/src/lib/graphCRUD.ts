// handlers/graphCRUD.ts
import { supabase } from "../lib/supabase"
import type { GraphNode, GraphRelationship } from "../lib/database.types"

export async function saveNode(nodeData: Partial<GraphNode>): Promise<void> {
  if (nodeData.id) {
    // Update existing node
    const { error: nodeError } = await supabase.from("nodes").update({ label: nodeData.label }).eq("id", nodeData.id)

    if (nodeError) throw nodeError

    // Replace properties
    await supabase.from("node_properties").delete().eq("node_id", nodeData.id)

    if (nodeData.properties && Object.keys(nodeData.properties).length > 0) {
      const propsToInsert = Object.entries(nodeData.properties).map(([key, value]) => ({
        node_id: nodeData.id!,
        key,
        value,
      }))

      const { error: propsError } = await supabase.from("node_properties").insert(propsToInsert)
      if (propsError) throw propsError
    }
  } else {
    // Create new node
    const { data: newNode, error: nodeError } = await supabase
      .from("nodes")
      .insert({ label: nodeData.label || "Node" })
      .select()
      .single()

    if (nodeError) throw nodeError

    if (nodeData.properties && Object.keys(nodeData.properties).length > 0) {
      const propsToInsert = Object.entries(nodeData.properties).map(([key, value]) => ({
        node_id: newNode.id,
        key,
        value,
      }))

      const { error: propsError } = await supabase.from("node_properties").insert(propsToInsert)
      if (propsError) throw propsError
    }
  }
}

export async function deleteNode(nodeId: string): Promise<void> {
  const { error } = await supabase.from("nodes").delete().eq("id", nodeId)
  if (error) throw error
}

export async function saveRelationship(relData: Partial<GraphRelationship>): Promise<void> {
  if (relData.id) {
    // Update existing
    const { error: relError } = await supabase.from("relationships").update({ type: relData.type }).eq("id", relData.id)

    if (relError) throw relError

    await supabase.from("relationship_properties").delete().eq("relationship_id", relData.id)

    if (relData.properties && Object.keys(relData.properties).length > 0) {
      const propsToInsert = Object.entries(relData.properties).map(([key, value]) => ({
        relationship_id: relData.id!,
        key,
        value,
      }))

      const { error: propsError } = await supabase.from("relationship_properties").insert(propsToInsert)
      if (propsError) throw propsError
    }
  } else {
    // Create new
    const { data: newRel, error: relError } = await supabase
      .from("relationships")
      .insert({
        type: relData.type || "",
        source_node_id: relData.source!,
        target_node_id: relData.target!,
      })
      .select()
      .single()

    if (relError) throw relError

    if (relData.properties && Object.keys(relData.properties).length > 0) {
      const propsToInsert = Object.entries(relData.properties).map(([key, value]) => ({
        relationship_id: newRel.id,
        key,
        value,
      }))

      const { error: propsError } = await supabase.from("relationship_properties").insert(propsToInsert)
      if (propsError) throw propsError
    }
  }
}

export async function deleteRelationship(relId: string): Promise<void> {
  const { error } = await supabase.from("relationships").delete().eq("id", relId)
  if (error) throw error
}
