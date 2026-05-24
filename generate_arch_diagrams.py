#!/usr/bin/env python3
"""
AgentMem Architecture Diagram Generator - FINAL VERSION
Professional architecture diagrams with proper layout
"""

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, Circle, FancyArrowPatch
import numpy as np

# Color palette
C = {
    'dark': '#1a1a2e', 'navy': '#16213e', 'blue': '#0f3460',
    'red': '#e94560', 'white': '#FFFFFF', 'gray': '#a0a0a0',
    'light': '#f5f5f5', 'cream': '#faf8f5',
    'client': '#2980B9',
    'api': '#27AE60',
    'unified': '#D4AC0D',
    'orchestration': '#8E44AD',
    'intelligence': '#C0392B',
    'storage': '#16A085',
    'cache': '#E67E22',
    'structured': '#5D6D7E',
    'vector': '#1ABC9C',
    'accent': '#3498DB',
    'core': '#2471A3',
    'semantic': '#7D3C98',
    'episodic': '#A93226',
    'working': '#D35400',
    'procedural': '#1E8449',
    'knowledge': '#148F77',
    'resource': '#784212',
    'contextual': '#566573',
}


def draw_rect(ax, x, y, w, h, color, title=None, subtitle=None, fontsize=11, title_color=None):
    """Draw a rounded rectangle with centered text"""
    if title_color is None:
        title_color = C['white']
    rect = FancyBboxPatch(xy=(x, y), width=w, height=h,
                         boxstyle='round,pad=0.02,rounding_size=0.08',
                         facecolor=color, edgecolor='#2C3E50', linewidth=1.5, alpha=0.95)
    ax.add_patch(rect)
    if title:
        ax.text(x + w/2, y + h*0.65, title, ha='center', va='center',
                fontsize=fontsize, fontweight='bold', color=title_color)
    if subtitle:
        ax.text(x + w/2, y + h*0.28, subtitle, ha='center', va='center',
                fontsize=fontsize-2, color='#BDC3C7', style='italic')


def draw_chip(ax, x, y, w, h, text, color, fontsize=8, text_color=None):
    """Draw a pill-shaped chip"""
    if text_color is None:
        text_color = C['white']
    rect = FancyBboxPatch(xy=(x, y), width=w, height=h,
                         boxstyle='round,pad=0.015,rounding_size=0.1',
                         facecolor=color, edgecolor='none', alpha=0.95)
    ax.add_patch(rect)
    ax.text(x + w/2, y + h/2, text, ha='center', va='center',
            fontsize=fontsize, fontweight='bold', color=text_color)


def draw_line(ax, x1, y1, x2, y2, color='#85929E'):
    """Draw a connecting line"""
    ax.plot([x1, x2], [y1, y2], color=color, linewidth=2, zorder=1)


def draw_arrow(ax, x1, y1, x2, y2, color='#85929E'):
    """Draw an arrow"""
    ax.annotate('', xy=(x2, y2), xytext=(x1, y1),
                arrowprops=dict(arrowstyle='->', color=color, lw=2.5,
                              connectionstyle='arc3,rad=0'),
                zorder=2)


def draw_layer(ax, label, y, x_max=18, color=None, fontsize=12):
    """Draw a layer label"""
    if color:
        ax.text(0.5, y, label, fontsize=fontsize, fontweight='bold',
                color=color, va='center', ha='left')


# ============================================================================
# 1. MAIN SYSTEM ARCHITECTURE
# ============================================================================
def create_main():
    fig, ax = plt.subplots(figsize=(20, 30))
    ax.set_xlim(0, 20)
    ax.set_ylim(0, 30)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    # Title
    ax.text(10, 29.2, 'AgentMem System Architecture', ha='center', fontsize=24,
            fontweight='bold', color='#2C3E50')
    ax.text(10, 28.6, 'Enterprise AI Memory Management Platform', ha='center',
            fontsize=12, color='#7F8C8D', style='italic')

    y = 27.5

    # === LAYER 1: CLIENT ===
    draw_layer(ax, '1. CLIENT LAYER', y, color=C['client'])
    for i, (name, color) in enumerate([('Python SDK', '#3776AB'), ('TypeScript SDK', '#F7DF1E'),
                                       ('Go SDK', '#00ADD8'), ('Cangjie SDK', '#0078D4')]):
        cx = 1 + i * 4.7
        draw_rect(ax, cx, y - 2.2, 4.2, 2, color, name, None, 12)
    y -= 4

    # Arrow down
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === LAYER 2: HTTP API ===
    draw_layer(ax, '2. HTTP REST API LAYER', y, color=C['api'])
    draw_rect(ax, 0.5, y - 3.5, 19, 3, C['api'], 'Axum Router', '175+ REST Endpoints', 16)
    mw_y = y - 1.5
    for i, m in enumerate(['CORS', 'Trace', 'Rate Limit', 'RBAC', 'Auth', 'Metrics']):
        draw_chip(ax, 1 + i * 3.1, mw_y, 2.8, 0.7, m, '#ECF0F1', 8, C['api'])
    y -= 5

    # Arrow down
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === LAYER 3: UNIFIED API ===
    draw_layer(ax, '3. UNIFIED API', y, color=C['unified'])
    draw_rect(ax, 2, y - 2.5, 16, 2, C['unified'], 'agent-mem (Memory::new())', None, 14)
    for i, m in enumerate(['add()', 'get()', 'del()', 'search()', 'update()']):
        ax.text(4 + i * 3, y - 1.5, m, ha='center', va='center',
                fontsize=10, color=C['white'], family='monospace')
    y -= 4

    # Arrow down
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === LAYER 4: ORCHESTRATION ===
    draw_layer(ax, '4. ORCHESTRATION LAYER', y, color=C['orchestration'])
    draw_rect(ax, 0.3, y - 5, 19.4, 4.5, C['orchestration'], 'MemoryEngine', None, 16)

    agents = [
        ('CoreAgent', 'Identity', C['core']), ('SemanticAgent', 'Knowledge', C['semantic']),
        ('EpisodicAgent', 'Events', C['episodic']), ('WorkingAgent', 'Active', C['working']),
        ('ProceduralAgent', 'Skills', C['procedural']), ('KnowledgeAgent', 'Graphs', C['knowledge']),
        ('ResourceAgent', 'Media', C['resource']), ('ContextualAgent', 'Situational', C['contextual']),
    ]
    for i, (name, desc, color) in enumerate(agents):
        row = i // 4
        col = i % 4
        bx = 0.8 + col * 4.7
        by = y - 4.2 - row * 2.2
        rect = FancyBboxPatch(xy=(bx, by), width=4.2, height=1.8,
                             boxstyle='round,pad=0.02,rounding_size=0.1',
                             facecolor=color, edgecolor='#2C3E50', linewidth=1.5)
        ax.add_patch(rect)
        ax.text(bx + 2.1, by + 1.2, name, ha='center', va='center',
                fontsize=11, fontweight='bold', color=C['white'])
        ax.text(bx + 2.1, by + 0.4, f'({desc})', ha='center', va='center',
                fontsize=9, color='#D5DBDB', style='italic')
    y -= 7

    # Arrow down
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === LAYER 5: INTELLIGENCE ===
    draw_layer(ax, '5. INTELLIGENCE LAYER', y, color=C['intelligence'])
    draw_rect(ax, 0.5, y - 4, 19, 3.5, C['intelligence'], 'agent-mem-intelligence', None, 16)
    components = ['Fact\nExtraction', 'Conflict\nResolution', 'Decision\nEngine', 'Importance\nScorer',
                  'Forgetting\nEngine', 'Metacognition', 'Auto-\nConsolidation', 'Reranking']
    for i, comp in enumerate(components):
        col = i % 4
        row = i // 4
        bx = 1 + col * 4.7
        by = y - 3.2 - row * 1.6
        rect = FancyBboxPatch(xy=(bx, by), width=4.2, height=1.3,
                             boxstyle='round,pad=0.01,rounding_size=0.08',
                             facecolor=C['white'], edgecolor='none', alpha=0.95)
        ax.add_patch(rect)
        ax.text(bx + 2.1, by + 0.65, comp, ha='center', va='center',
                fontsize=9, fontweight='bold', color=C['intelligence'])
    y -= 5.5

    # Arrow down
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === LAYER 6: STORAGE ===
    draw_layer(ax, '6. STORAGE ABSTRACTION', y, color=C['storage'])
    draw_rect(ax, 0.3, y - 7, 19.4, 6.5, C['storage'], 'StorageFactory (14+ Backends)', None, 16)

    # L1 Cache
    draw_rect(ax, 0.8, y - 3.5, 4, 3, C['cache'], 'L1 Cache', 'In-Memory LRU', 12)
    # L2 Cache
    draw_rect(ax, 0.8, y - 6.2, 4, 2.5, C['cache'], 'L2 Cache', 'Redis', 12)

    # Structured Storage
    draw_rect(ax, 5.5, y - 6.5, 6.5, 6, C['structured'], 'Structured Storage', None, 13)
    for i, s in enumerate(['LibSQL', 'PostgreSQL', 'MongoDB', 'MySQL', 'Redis']):
        ax.text(6, y - 1.5 - i * 1, f'● {s}', ha='left', va='center',
                fontsize=9, fontweight='bold', color=C['white'])

    # Vector Storage
    draw_rect(ax, 13, y - 6.5, 6.5, 6, C['vector'], 'Vector Storage', None, 13)
    vecs = ['LanceDB', 'Qdrant', 'Pinecone', 'FAISS', 'pgvector', 'Chroma', 'Weaviate', 'Milvus', 'Supabase']
    for i, v in enumerate(vecs):
        col = i % 2
        row = i // 2
        ax.text(13.5 + col * 3, y - 1.5 - row * 1.2, f'● {v}', ha='left', va='center',
                fontsize=8, fontweight='bold', color=C['white'])

    plt.tight_layout()
    return fig


# ============================================================================
# 2. MEMORYV4 MODEL
# ============================================================================
def create_memory():
    fig, ax = plt.subplots(figsize=(18, 18))
    ax.set_xlim(0, 18)
    ax.set_ylim(0, 18)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(9, 17.3, 'MemoryV4 Core Architecture', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(9, 16.7, 'Multimodal • Open Attribute • Relation Graph', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    # Main Memory Box
    draw_rect(ax, 0.5, 13, 17, 3.5, C['intelligence'], 'MemoryV4', None, 20)
    fields = [('id', 'String', 'Unique identifier'), ('content', 'Content', 'Multimodal data'),
             ('attributes', 'AttributeSet', 'Open schema'), ('relations', 'RelationGraph', 'Weighted relations'),
             ('metadata', 'Metadata', 'Auditing')]
    for i, (n, t, d) in enumerate(fields):
        ax.text(1, 15.8 - i * 0.6, n, fontsize=11, fontweight='bold', color=C['white'], family='monospace')
        ax.text(3.5, 15.8 - i * 0.6, f': {t}', fontsize=11, color='#F4D03F')
        ax.text(7.5, 15.8 - i * 0.6, f'// {d}', fontsize=9, color='#AED6F1', style='italic')

    # Content Box
    draw_rect(ax, 0.5, 8, 5.5, 4.5, C['core'], 'Content', '(Multimodal)', 14)
    for i, ct in enumerate(['Text', 'Image', 'Audio', 'Video', 'Structured', 'Mixed']):
        col = i % 2
        row = i // 2
        ax.text(1, 11.8 - row * 1.4, f'● {ct}', fontsize=10, fontweight='bold', color=C['white'], va='center')

    # AttributeSet Box
    draw_rect(ax, 6.5, 8, 5, 4.5, C['orchestration'], 'AttributeSet', '(Open Schema)', 14)
    ax.text(7, 12, 'namespace: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(7, 11.2, 'name: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(7, 10.4, 'value: AttributeValue', fontsize=9, color=C['white'], family='monospace')
    ax.text(7, 9.2, 'Namespaces:', fontsize=10, fontweight='bold', color=C['white'])
    for i, ns in enumerate(['system', 'user', 'domain']):
        ax.text(7.5 + (i % 2) * 2, 8.5 - (i // 2) * 0.5, f'• {ns}', fontsize=8, color='#AED6F1')

    # Metadata Box
    draw_rect(ax, 12, 8, 5.5, 4.5, C['storage'], 'Metadata', '(Auditing)', 14)
    meta = ['created_at: DateTime', 'updated_at: DateTime', 'access_count: u64',
            'last_accessed: Option<DT>', 'importance: f32']
    for i, m in enumerate(meta):
        ax.text(12.5, 12 - i * 0.65, m, fontsize=8, color=C['white'], family='monospace')

    # Arrows
    draw_arrow(ax, 9, 13, 3.25, 12.5)
    draw_arrow(ax, 9, 13, 9, 12.5)
    draw_arrow(ax, 9, 13, 14.75, 12.5)

    # RelationGraph
    draw_rect(ax, 1.5, 1, 15, 6.5, C['unified'], 'RelationGraph (Typed Weighted Relationships)', None, 16)
    rels = [('References', 'Links to memory'), ('Supersedes', 'Updates old'),
           ('PartOf', 'Composition'), ('SimilarTo', 'Semantic'),
           ('CausedBy', 'Causal'), ('Custom(...)', 'User-defined')]
    for i, (n, d) in enumerate(rels):
        col = i % 3
        row = i // 3
        bx = 2.5 + col * 5
        by = 6 - row * 2.5
        rect = FancyBboxPatch(xy=(bx, by), width=4.5, height=2,
                             boxstyle='round,pad=0.02,rounding_size=0.1',
                             facecolor=C['accent'], edgecolor='none', alpha=0.95)
        ax.add_patch(rect)
        ax.text(bx + 2.25, by + 1.3, n, ha='center', va='center',
                fontsize=11, fontweight='bold', color=C['white'])
        ax.text(bx + 2.25, by + 0.4, d, ha='center', va='center',
                fontsize=8, color='#D5DBDB', style='italic')

    draw_arrow(ax, 9, 8, 9, 7.5)

    plt.tight_layout()
    return fig


# ============================================================================
# 3. SEARCH ENGINE
# ============================================================================
def create_search():
    fig, ax = plt.subplots(figsize=(16, 16))
    ax.set_xlim(0, 16)
    ax.set_ylim(0, 16)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(8, 15.3, 'Hybrid Search Engine (V2)', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(8, 14.7, '5 Search Modes • Reciprocal Rank Fusion', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    # Query
    draw_rect(ax, 3, 13.5, 10, 1.4, '#2C3E50', 'User Query: "nginx setup"', None, 13)

    # Engines
    engines = [('Vector Search', 'Semantic', '#2980B9', 'cosine_sim()'),
               ('BM25 Search', 'Keyword', '#7D3C98', 'bm25_score()'),
               ('Full-Text', 'Fuzzy', '#CA6F1E', 'edit_dist()')]
    for i, (n, d, col, f) in enumerate(engines):
        ex = 1 + i * 5
        draw_rect(ax, ex, 10, 4.5, 3, col, n, d, 13)
        ax.text(ex + 2.25, 11.5, f, ha='center', va='center',
                fontsize=9, color=C['white'], family='monospace')
        draw_arrow(ax, 8, 13.5, ex + 2.25, 13)

    # RRF
    draw_rect(ax, 2, 5.5, 12, 4, C['intelligence'], 'RRF Reranking (Reciprocal Rank Fusion)', None, 17)
    ax.text(8, 8.8, 'Score = Σ 1/(k + rank_i) where k = 60', ha='center',
            fontsize=11, color='#F4D03F', family='monospace')
    ax.text(8, 7.8, 'Combines multiple ranking signals', ha='center',
            fontsize=9, color='#D5DBDB', style='italic')

    for i, (_, _, col, _) in enumerate(engines):
        draw_arrow(ax, 1 + i * 5 + 2.25, 10, 3.5 + i * 3.5, 9.5)

    # Results
    draw_rect(ax, 2, 1.5, 12, 3.5, C['storage'], 'Ranked Results', None, 15)
    results = [('1', 'Nginx SSL/TLS config', '0.95'), ('2', 'Reverse proxy guide', '0.89'),
               ('3', 'SSL certificate install', '0.82'), ('4', 'Location blocks', '0.78')]
    for i, (rank, title, score) in enumerate(results):
        col = i % 2
        row = i // 2
        draw_chip(ax, 3 + col * 6, 3.5 - row * 1.5, 0.6, 0.6, rank, C['accent'])
        ax.text(4 + col * 6, 3.8 - row * 1.5, title, fontsize=10, color=C['white'], va='center')
        ax.text(10 + col * 6, 3.8 - row * 1.5, f'({score})', fontsize=10, color='#F4D03F', va='center')

    draw_arrow(ax, 8, 5.5, 8, 5)

    # Engine comparison
    ax.text(8, 0.8, '5 Engines: Vector (semantic) | BM25 (keyword) | Full-Text (fuzzy) | Fuzzy (spelling) | Hybrid (RRF)',
             ha='center', fontsize=8, color='#7F8C8D')

    plt.tight_layout()
    return fig


# ============================================================================
# 4. MULTI-AGENT
# ============================================================================
def create_agents():
    fig, ax = plt.subplots(figsize=(18, 16))
    ax.set_xlim(0, 18)
    ax.set_ylim(0, 16)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(9, 15.3, 'Multi-Agent Coordination Architecture', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')

    # Orchestrator
    draw_rect(ax, 5, 13, 8, 1.6, C['orchestration'], 'Orchestrator (MemoryEngine)', None, 16)

    # Agents
    agents = [('CoreAgent', 'Identity', C['core']), ('SemanticAgent', 'Knowledge', C['semantic']),
             ('EpisodicAgent', 'Events', C['episodic']), ('WorkingAgent', 'Active', C['working']),
             ('ProceduralAgent', 'Skills', C['procedural']), ('KnowledgeAgent', 'Graphs', C['knowledge']),
             ('ResourceAgent', 'Media', C['resource']), ('ContextualAgent', 'Situational', C['contextual'])]

    for i, (name, desc, color) in enumerate(agents):
        row = i // 4
        col = i % 4
        bx = 1 + col * 4.2
        by = 9.5 - row * 4
        rect = FancyBboxPatch(xy=(bx, by), width=3.8, height=3.2,
                             boxstyle='round,pad=0.03,rounding_size=0.12',
                             facecolor=color, edgecolor='#2C3E50', linewidth=2)
        ax.add_patch(rect)
        ax.text(bx + 1.9, by + 2.2, name, ha='center', va='center',
                fontsize=12, fontweight='bold', color=C['white'])
        ax.text(bx + 1.9, by + 0.8, f'({desc})', ha='center', va='center',
                fontsize=9, color='#D5DBDB', style='italic')
        draw_arrow(ax, 9, 13, bx + 1.9, by + 3.2, C['orchestration'])

    # Event Bus
    draw_rect(ax, 2, 5, 14, 1.4, C['unified'], 'Event Bus / Message Bus', 'Pub/Sub • Inter-Agent', 13)

    # State Machine
    draw_rect(ax, 14, 2.5, 3.5, 4.5, C['dark'], 'AgentState', None, 13)
    states = [('Idle', 15.25, 6.3, '#27AE60'), ('Thinking', 15.75, 5.3, '#F39C12'),
             ('Executing', 16.25, 6.3, '#2980B9'), ('Waiting', 16.25, 4.3, '#7D3C98'),
             ('Error', 15.75, 3.3, '#C0392B')]
    for name, x, y, color in states:
        circle = Circle((x, y), 0.3, facecolor=color, edgecolor='white', linewidth=2)
        ax.add_patch(circle)
        ax.text(x, y, name[:2], ha='center', va='center', fontsize=9, fontweight='bold', color='white')

    plt.tight_layout()
    return fig


# ============================================================================
# 5. STORAGE
# ============================================================================
def create_storage():
    fig, ax = plt.subplots(figsize=(18, 18))
    ax.set_xlim(0, 18)
    ax.set_ylim(0, 18)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(9, 17.3, 'Storage Architecture', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(9, 16.7, 'UnifiedStorageCoordinator • Repository Pattern • 14+ Backends', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    # Coordinator
    draw_rect(ax, 3, 14.5, 12, 1.5, C['storage'], 'UnifiedStorageCoordinator', None, 15)

    # L1 Cache
    draw_rect(ax, 0.5, 12, 4.5, 2.5, C['cache'], 'L1 Cache', 'In-Memory LRU', 13)
    ax.text(2.75, 13.9, 'Hot data', ha='center', fontsize=9, color='#D5DBDB', style='italic')
    ax.text(2.75, 13.3, 'LRU eviction', ha='center', fontsize=8, color='#AED6F1')

    # L2 Cache
    draw_rect(ax, 0.5, 9, 4.5, 2.5, C['cache'], 'L2 Cache', 'Redis', 13)
    ax.text(2.75, 10.9, 'Distributed', ha='center', fontsize=9, color='#D5DBDB', style='italic')
    ax.text(2.75, 10.3, 'Optional', ha='center', fontsize=8, color='#AED6F1')

    # Structured Storage
    draw_rect(ax, 5.5, 4, 6, 8, C['structured'], 'Structured Storage', None, 15)
    stores = [('EpisodicMemoryStore', 'Event-based memories'),
              ('SemanticMemoryStore', 'Factual knowledge'),
              ('ProceduralMemoryStore', 'Procedures & tasks'),
              ('CoreMemoryStore', 'Identity/persona'),
              ('WorkingMemoryStore', 'Session context'),
              ('MessageStore', 'Chat history'),
              ('ToolStore', 'Tool registry')]
    for i, (n, d) in enumerate(stores):
        ax.text(5.8, 11.3 - i * 1, f'• {n}', ha='left', va='center',
                fontsize=9, fontweight='bold', color=C['white'])
        ax.text(5.8, 11 - i * 1, f'  {d}', ha='left', va='center',
                fontsize=7, color='#AED6F1')

    # Backend chips
    rect1 = FancyBboxPatch(xy=(5.8, 4.5), width=2, height=0.8,
                           boxstyle='round,pad=0.01', facecolor='#7B8896', edgecolor='none')
    ax.add_patch(rect1)
    ax.text(6.8, 4.9, 'LibSQL', ha='center', fontsize=7, color=C['white'])

    rect2 = FancyBboxPatch(xy=(8, 4.5), width=2, height=0.8,
                           boxstyle='round,pad=0.01', facecolor='#7B8896', edgecolor='none')
    ax.add_patch(rect2)
    ax.text(9, 4.9, 'PostgreSQL', ha='center', fontsize=7, color=C['white'])

    rect3 = FancyBboxPatch(xy=(10.2, 4.5), width=1.3, height=0.8,
                           boxstyle='round,pad=0.01', facecolor='#7B8896', edgecolor='none')
    ax.add_patch(rect3)
    ax.text(10.85, 4.9, 'MongoDB', ha='center', fontsize=7, color=C['white'])

    # Vector Storage
    draw_rect(ax, 12, 4, 5.5, 8, C['vector'], 'Vector Storage', None, 15)
    vecs = [('LanceDB', 'Local', '#1ABC9C'), ('Qdrant', 'Cloud', '#16A085'),
            ('Pinecone', 'Managed', '#148F77'), ('FAISS', 'Meta', '#117A65'),
            ('pgvector', 'Postgres', '#0E6655'), ('Chroma', 'Embeddings', '#1ABC9C'),
            ('Weaviate', 'Hybrid', '#16A085'), ('Milvus', 'Scale', '#148F77'),
            ('Supabase', 'PG AI', '#0E6655')]
    for i, (n, d, col) in enumerate(vecs):
        col_idx = i % 2
        row_idx = i // 2
        bx = 12.3 + col_idx * 2.5
        by = 11.3 - row_idx * 1.8
        rect = FancyBboxPatch(xy=(bx, by), width=2.3, height=1.5,
                             boxstyle='round,pad=0.01', facecolor=C['white'], edgecolor='none', alpha=0.95)
        ax.add_patch(rect)
        ax.text(bx + 1.15, by + 1, n, ha='center', va='center',
                fontsize=8, fontweight='bold', color=col)
        ax.text(bx + 1.15, by + 0.4, d, ha='center', va='center',
                fontsize=6, color='#7F8C8D')

    # Arrows
    draw_arrow(ax, 9, 14.5, 2.75, 14.5)
    draw_arrow(ax, 9, 14.5, 2.75, 11.5)
    draw_arrow(ax, 9, 14.5, 9, 12)
    draw_arrow(ax, 9, 14.5, 14.75, 12)

    # Backend Trait
    draw_rect(ax, 0.5, 2.5, 17, 1.5, C['dark'], 'StorageBackend Trait', None, 12)
    ax.text(9, 3.5, 'async fn store() / retrieve() / search() / delete() / batch()', ha='center',
            fontsize=9, color='#F4D03F', family='monospace')

    # Coordinator features
    ax.text(9, 1.8, 'Features: L1 (LRU) → L2 (Redis) → Structured → Vector | Batch queue | Auto-compression',
            ha='center', fontsize=8, color='#7F8C8D')

    plt.tight_layout()
    return fig


# ============================================================================
# 6. SDK
# ============================================================================
def create_sdk():
    fig, ax = plt.subplots(figsize=(18, 14))
    ax.set_xlim(0, 18)
    ax.set_ylim(0, 14)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(9, 13.3, 'Multi-Language SDK Architecture', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')

    # Server
    draw_rect(ax, 4, 11.5, 10, 1.3, '#2C3E50', 'AgentMem Server (HTTP REST API)', None, 13)

    # SDKs
    sdks = [('Python SDK', 'sdks/python/', '#3776AB', ['httpx.AsyncClient', 'Pydantic models', 'Retry + cache']),
            ('TypeScript SDK', 'sdks/js/', '#F7DF1E', ['Fetch API', 'TypeScript types', 'Zod validation']),
            ('Go SDK', 'sdks/go/', '#00ADD8', ['net/http', 'Context support', 'Generated types']),
            ('Cangjie SDK', 'sdks/cj/', '#0078D4', ['Native binding', '.cj package', 'Type safe'])]

    for i, (name, path, color, feats) in enumerate(sdks):
        sx = 0.8 + i * 4.2
        draw_rect(ax, sx, 5.5, 4, 5.5, color, name, path, 12)
        tc = 'black' if color == '#F7DF1E' else C['white']
        for j, f in enumerate(feats):
            ax.text(sx + 0.4, 10.3 - j * 0.6, f'• {f}', fontsize=9, color=tc)
        draw_arrow(ax, sx + 2, 5.5, 9, 11.5, color)

    # API Methods
    draw_rect(ax, 1, 4.2, 16, 1, C['api'], 'Common API Methods', None, 12)
    methods = ['add_memory()', 'search()', 'get_memory()', 'list_memories()', 'delete()']
    for i, m in enumerate(methods):
        draw_chip(ax, 1.5 + i * 3.2, 4.5, 3, 0.55, m, C['white'], 8, C['api'])

    # Core Client
    draw_rect(ax, 1, 2, 16, 1.8, C['storage'], 'AgentMemClient Core', None, 12)
    comps = [('HTTP Client', 'Pool'), ('Request Cache', 'TTL'), ('Retry Logic', 'Backoff'), ('Type Models', 'Pydantic')]
    for i, (n, d) in enumerate(comps):
        draw_chip(ax, 2 + i * 4, 2.4, 3.5, 1, n, C['white'], 9, C['storage'])

    draw_arrow(ax, 9, 4.2, 9, 3.8)

    plt.tight_layout()
    return fig


# ============================================================================
# 7. MCP
# ============================================================================
def create_mcp():
    fig, ax = plt.subplots(figsize=(18, 16))
    ax.set_xlim(0, 18)
    ax.set_ylim(0, 16)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(9, 15.3, 'MCP (Model Context Protocol) Integration', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(9, 14.7, 'examples/mcp-stdio-server/ • 5 Core Tools • Multi-Transport', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    # Server
    draw_rect(ax, 4, 13, 10, 1.5, C['orchestration'], 'MCP Server', 'Tool Registry • Prompts • Resources', 15)

    # Transport
    draw_rect(ax, 0.5, 11, 17, 1.5, C['dark'], 'Transport Adapters', None, 13)
    trans = [('stdio', 'Default (stdio)', '#27AE60'), ('HTTP', 'Optional', '#2980B9'),
             ('SSE', 'Server-Sent Events', '#7D3C98'), ('WebSocket', 'Real-time', '#CA6F1E')]
    for i, (n, d, col) in enumerate(trans):
        draw_chip(ax, 1 + i * 4.25, 11.3, 4, 0.8, f'{n}', col)
        ax.text(3 + i * 4.25, 11.65, d, ha='center', fontsize=7, color='#AED6F1', style='italic')
    draw_arrow(ax, 9, 13, 9, 12.5)

    # Tools
    draw_rect(ax, 0.5, 7, 17, 3.5, C['intelligence'], 'MCP Tools (5 Core)', None, 15)
    tools = [
        ('memory_add', 'Add new memory', 'Input: content, type, metadata'),
        ('memory_search', 'Semantic search', 'Input: query, filters, limit'),
        ('memory_chat', 'Agent chat', 'Input: message, context'),
        ('system_prompt', 'Build context', 'Input: agent_id, scope'),
        ('list_agents', 'Agent listing', 'Returns: agent[] with stats'),
    ]
    for i, (n, d, inp) in enumerate(tools):
        rect = FancyBboxPatch(xy=(1 + i * 3.4, 7.3), width=3.2, height=2.9,
                             boxstyle='round,pad=0.02,rounding_size=0.1',
                             facecolor=C['white'], edgecolor='#2C3E50', linewidth=1.5)
        ax.add_patch(rect)
        ax.text(1 + i * 3.4 + 1.6, 9.5, n, ha='center', va='center',
                fontsize=8, fontweight='bold', color=C['intelligence'], family='monospace')
        ax.text(1 + i * 3.4 + 1.6, 8.7, d, ha='center', va='center',
                fontsize=10, fontweight='bold', color='#2C3E50')
        ax.text(1 + i * 3.4 + 1.6, 7.8, inp, ha='center', va='center',
                fontsize=7, color='#7F8C8D', style='italic')
    draw_arrow(ax, 9, 11, 9, 10.5)

    # Auth
    draw_rect(ax, 0.5, 5, 5.5, 1.8, C['storage'], 'Authentication', None, 12)
    draw_chip(ax, 0.8, 5.8, 2.4, 0.7, 'JWT Token', '#27AE60')
    draw_chip(ax, 3.4, 5.8, 2.4, 0.7, 'API Key', '#27AE60')
    ax.text(3.25, 5.3, 'Secure token-based', ha='center', fontsize=7, color='#AED6F1', style='italic')
    draw_arrow(ax, 3, 7, 3.25, 6.8)

    # AI Providers
    draw_rect(ax, 7, 5, 10.5, 1.8, C['unified'], 'AI Provider Integration', None, 12)
    provs = ['OpenAI', 'Anthropic', 'DeepSeek', 'Gemini', 'Azure']
    for i, p in enumerate(provs):
        draw_chip(ax, 7.5 + i * 2, 5.8, 1.9, 0.7, p, C['accent'])
    ax.text(12.25, 5.3, '+ 15 more providers', ha='center', fontsize=7, color='#AED6F1', style='italic')
    draw_arrow(ax, 9, 7, 12.25, 6.8)

    # Features
    draw_rect(ax, 0.5, 3, 17, 1.5, C['navy'], 'Features', None, 11)
    feats = ['Tool registry', 'Prompt templates', 'Resource mgmt', 'Streaming', 'Error handling']
    for i, f in enumerate(feats):
        draw_chip(ax, 1 + i * 3.4, 3.4, 3.1, 0.8, f, C['client'])

    # Connection
    draw_rect(ax, 5, 1.2, 8, 1.3, C['red'], 'Connect to AI Agents', None, 11)

    plt.tight_layout()
    return fig


# ============================================================================
# 8. LLM
# ============================================================================
def create_llm():
    fig, ax = plt.subplots(figsize=(18, 14))
    ax.set_xlim(0, 18)
    ax.set_ylim(0, 14)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(9, 13.3, 'LLM Provider Integration', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')

    # Trait
    draw_rect(ax, 4, 11.5, 10, 1.3, C['intelligence'], 'LLMClient Trait', None, 14)
    ax.text(9, 11.9, 'async fn complete() / chat() / embed()', ha='center',
            fontsize=9, color='#F4D03F', family='monospace')

    # Providers
    providers = [('OpenAI', 'GPT-4, GPT-3.5', '#412991'), ('Anthropic', 'Claude 3, 2', '#CC785C'),
                 ('DeepSeek', 'DeepSeek Chat', '#0E6655'), ('Google', 'Gemini Pro', '#4285F4'),
                 ('Azure', 'Azure OpenAI', '#0078D4'), ('Mistral', 'Mistral Large', '#3A1F5D'),
                 ('Groq', 'Fast inference', '#E65100'), ('Ollama', 'Local models', '#27AE60'),
                 ('TogetherAI', 'Open models', '#9B59B6'), ('Cohere', 'Embeddings', '#2C3E50'),
                 ('Bedrock', 'AWS models', '#FF9900'), ('Zhipu', 'GLM models', '#E53935')]

    for i, (n, m, col) in enumerate(providers):
        row = i // 4
        col_idx = i % 4
        bx = 1.5 + col_idx * 4.2
        by = 9 - row * 3.5
        draw_rect(ax, bx, by, 4, 3, col, n, m, 12)

    # Embeddings
    draw_rect(ax, 0.5, 1, 17, 1.8, C['unified'], 'Embedding Providers', None, 12)
    emb = ['FastEmbed (default)', 'OpenAI', 'Cohere', 'Custom ONNX']
    for i, e in enumerate(emb):
        draw_chip(ax, 1 + i * 4.3, 1.4, 4, 0.9, e, C['accent'])

    plt.tight_layout()
    return fig


# ============================================================================
# 9. PLUGIN
# ============================================================================
def create_plugin():
    fig, ax = plt.subplots(figsize=(16, 12))
    ax.set_xlim(0, 16)
    ax.set_ylim(0, 12)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(8, 11.3, 'WASM Plugin System', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')

    draw_rect(ax, 4, 9.5, 8, 1.3, C['orchestration'], 'Plugin SDK', None, 14)
    draw_rect(ax, 2, 6.5, 12, 2.5, C['intelligence'], 'PluginManager', 'WASM Runtime • Hot-reload • Sandboxing', 15)

    plugins = [('Plugin1', 'Custom'), ('Plugin2', 'Custom'), ('Plugin3', 'Custom'), ('Plugin4', 'Custom')]
    for i, (n, d) in enumerate(plugins):
        draw_chip(ax, 3 + i * 3, 7.2, 2.5, 1.4, n, C['accent'])

    draw_arrow(ax, 8, 9.5, 8, 9)

    draw_rect(ax, 1, 4, 14, 2, C['storage'], 'Performance Metrics', None, 12)
    metrics = [('216K', 'ops/sec'), ('<100ms', 'latency'), ('93,000x', 'cache')]
    for i, (v, d) in enumerate(metrics):
        ax.text(2 + i * 5, 5.3, v, ha='center', fontsize=18, fontweight='bold', color=C['white'])
        ax.text(2 + i * 5, 4.7, d, ha='center', fontsize=9, color='#AED6F1', style='italic')

    draw_rect(ax, 1, 1.5, 14, 2, C['dark'], 'Example: #[plugin] pub fn custom_tool(ctx: &Context, input: &str) -> Result<String>',
             None, 9)

    plt.tight_layout()
    return fig


# ============================================================================
# 10. FILE-CENTRIC
# ============================================================================
def create_file_centric():
    fig, ax = plt.subplots(figsize=(16, 14))
    ax.set_xlim(0, 16)
    ax.set_ylim(0, 14)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(8, 13.3, 'File-Centric Architecture (Phase D1)', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')

    # Resource Descriptor
    draw_rect(ax, 0.5, 10.5, 7, 2.3, C['core'], 'ResourceDescriptor', None, 14)
    ax.text(1, 12.3, 'id: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(1, 11.7, 'uri: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(1, 11.1, 'media_type: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(1, 10.5, 'status: ResourceStatus', fontsize=9, color=C['white'], family='monospace')

    # Category Descriptor
    draw_rect(ax, 8.5, 10.5, 7, 2.3, C['semantic'], 'CategoryDescriptor', None, 14)
    ax.text(9, 12.3, 'id: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(9, 11.7, 'path: String', fontsize=9, color=C['white'], family='monospace')
    ax.text(9, 11.1, 'parent_id: Option<String>', fontsize=9, color=C['white'], family='monospace')
    ax.text(9, 10.5, 'item_count: i64', fontsize=9, color=C['white'], family='monospace')

    # Pipeline
    draw_rect(ax, 1, 6.5, 14, 3.5, C['intelligence'], 'Extraction Pipeline', None, 15)
    steps = [('File', 'Upload'), ('Resource', 'Mount'), ('Parse', 'Content'),
             ('Extract', 'Entities'), ('Memory', 'Store')]
    for i, (n, d) in enumerate(steps):
        rect = FancyBboxPatch(xy=(2 + i * 2.9, 7), width=2.5, height=2.5,
                             boxstyle='round,pad=0.02,rounding_size=0.1',
                             facecolor=C['accent'], edgecolor='none', alpha=0.95)
        ax.add_patch(rect)
        ax.text(2 + i * 2.9 + 1.25, 8, n, ha='center', va='center',
                fontsize=11, fontweight='bold', color=C['white'])
        ax.text(2 + i * 2.9 + 1.25, 7.2, d, ha='center', va='center',
                fontsize=8, color='#D5DBDB', style='italic')
        if i < 4:
            ax.annotate('', xy=(4.8 + i * 2.9, 8.25), xytext=(4.5 + i * 2.9, 8.25),
                       arrowprops=dict(arrowstyle='->', color='#85929E', lw=2))

    # Scope
    draw_rect(ax, 1, 4.5, 14, 1.5, C['storage'], 'ScopeDescriptor', None, 12)
    scopes = ['agent_id: String', 'user_id: Option<String>', 'session_id: Option<String>', 'organization_id: Option<String>']
    for i, s in enumerate(scopes):
        draw_chip(ax, 1.5 + i * 3.5, 4.8, 3.2, 0.8, s, C['cache'])

    # Status Flow
    draw_rect(ax, 1, 2, 14, 2, C['unified'], 'Resource Status Flow', None, 12)
    statuses = ['PENDING', 'MOUNTED', 'EXTRACTING', 'SUCCEEDED', 'FAILED']
    for i, s in enumerate(statuses):
        draw_chip(ax, 2 + i * 2.9, 2.5, 2.5, 0.9, s, C['accent'])

    plt.tight_layout()
    return fig


# ============================================================================
# MAIN
# ============================================================================
def main():
    output_dir = '/Users/louloulin/Documents/linchong/cjproject/contextengine/agentmen'

    diagrams = [
        ('arch101_main.png', create_main, 'Main Architecture'),
        ('arch101_memory.png', create_memory, 'MemoryV4'),
        ('arch101_search.png', create_search, 'Search Engine'),
        ('arch101_agents.png', create_agents, 'Multi-Agent'),
        ('arch101_storage.png', create_storage, 'Storage'),
        ('arch101_sdk.png', create_sdk, 'SDK'),
        ('arch101_mcp.png', create_mcp, 'MCP'),
        ('arch101_llm.png', create_llm, 'LLM'),
        ('arch101_plugin.png', create_plugin, 'Plugin'),
        ('arch101_file_centric.png', create_file_centric, 'File-Centric'),
    ]

    print("Generating comprehensive architecture diagrams...")
    for filename, func, desc in diagrams:
        try:
            fig = func()
            fig.savefig(f'{output_dir}/{filename}', dpi=150, bbox_inches='tight',
                       facecolor=C['cream'], edgecolor='none')
            plt.close(fig)
            print(f"  ✓ {filename} ({desc})")
        except Exception as e:
            print(f"  ✗ {filename}: {e}")
            import traceback
            traceback.print_exc()

    print("\nAll diagrams generated!")


if __name__ == '__main__':
    main()