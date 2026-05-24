#!/usr/bin/env python3
"""
AgentMem Architecture Diagram Generator - UI ANALYSIS VERSION
Includes comprehensive UI completeness analysis with gaps highlighted
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
    # Status colors
    'complete': '#27AE60',   # Green - complete
    'partial': '#F39C12',     # Orange - partial
    'missing': '#C0392B',     # Red - missing/not implemented
    'placeholder': '#95A5A6', # Gray - placeholder
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


def draw_arrow(ax, x1, y1, x2, y2, color='#85929E'):
    """Draw an arrow"""
    ax.annotate('', xy=(x2, y2), xytext=(x1, y1),
                arrowprops=dict(arrowstyle='->', color=color, lw=2.5,
                              connectionstyle='arc3,rad=0'),
                zorder=2)


def draw_status_badge(ax, x, y, status, fontsize=8):
    """Draw a status badge (Complete/Partial/Missing)"""
    status_colors = {
        'COMPLETE': C['complete'],
        'PARTIAL': C['partial'],
        'MISSING': C['missing'],
        'PLACEHOLDER': C['placeholder']
    }
    status_text = {
        'COMPLETE': '[OK] COMPLETE',
        'PARTIAL': '[!] PARTIAL',
        'MISSING': '[X] MISSING',
        'PLACEHOLDER': '[.] PLACEHOLDER'
    }
    color = status_colors.get(status, C['gray'])
    text = status_text.get(status, status)
    ax.text(x, y, text, fontsize=fontsize, color=color, fontweight='bold')


# ============================================================================
# MAIN SYSTEM ARCHITECTURE WITH UI STATUS
# ============================================================================
def create_main():
    fig, ax = plt.subplots(figsize=(22, 32))
    ax.set_xlim(0, 22)
    ax.set_ylim(0, 32)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    # Title
    ax.text(11, 31.2, 'AgentMem System Architecture', ha='center', fontsize=24,
            fontweight='bold', color='#2C3E50')
    ax.text(11, 30.6, 'Enterprise AI Memory Management Platform', ha='center',
            fontsize=12, color='#7F8C8D', style='italic')

    y = 29.5

    # === LAYER 1: CLIENT ===
    draw_rect(ax, 0.5, y-1, 21, 1, '#2C3E50', 'CLIENT LAYER', None, 14)
    draw_status_badge(ax, 18, y-0.3, 'COMPLETE', 9)
    y -= 1.5

    for i, (name, color) in enumerate([('Python SDK', '#3776AB'), ('TypeScript SDK', '#F7DF1E'),
                                       ('Go SDK', '#00ADD8'), ('Cangjie SDK', '#0078D4')]):
        cx = 1 + i * 5.2
        draw_rect(ax, cx, y - 2.5, 4.8, 2.3, color, name, None, 12)
        draw_status_badge(ax, cx + 3.5, y - 0.5, 'COMPLETE', 7)
    y -= 4

    # Arrow down
    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === LAYER 2: HTTP API ===
    draw_rect(ax, 0.5, y-3.5, 21, 3, C['api'], 'HTTP REST API LAYER', '175+ REST Endpoints', 16)
    draw_status_badge(ax, 18, y-0.5, 'COMPLETE', 9)
    mw_y = y - 1.5
    for i, m in enumerate(['CORS', 'Trace', 'Rate Limit', 'RBAC', 'Auth', 'Metrics']):
        draw_chip(ax, 1 + i * 3.5, mw_y, 3.2, 0.7, m, '#ECF0F1', 8, C['api'])
    y -= 5

    # Arrow down
    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === LAYER 3: UNIFIED API ===
    draw_rect(ax, 2, y-2.5, 18, 2, C['unified'], 'agent-mem (Memory::new())', None, 14)
    draw_status_badge(ax, 17.5, y-0.5, 'COMPLETE', 9)
    for i, m in enumerate(['add()', 'get()', 'del()', 'search()', 'update()']):
        ax.text(4.5 + i * 3.2, y - 1.5, m, ha='center', va='center',
                fontsize=10, color=C['white'], family='monospace')
    y -= 4

    # Arrow down
    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === LAYER 4: ORCHESTRATION ===
    draw_rect(ax, 0.3, y-5.5, 21.4, 5, C['orchestration'], 'MemoryEngine (8 Specialized Agents)', None, 16)
    draw_status_badge(ax, 18.5, y-0.5, 'COMPLETE', 9)

    agents = [
        ('CoreAgent', 'Identity', C['core']), ('SemanticAgent', 'Knowledge', C['semantic']),
        ('EpisodicAgent', 'Events', C['episodic']), ('WorkingAgent', 'Active', C['working']),
        ('ProceduralAgent', 'Skills', C['procedural']), ('KnowledgeAgent', 'Graphs', C['knowledge']),
        ('ResourceAgent', 'Media', C['resource']), ('ContextualAgent', 'Situational', C['contextual']),
    ]
    for i, (name, desc, color) in enumerate(agents):
        row = i // 4
        col = i % 4
        bx = 0.8 + col * 5.2
        by = y - 5 - row * 2.5
        rect = FancyBboxPatch(xy=(bx, by), width=4.8, height=2.2,
                             boxstyle='round,pad=0.02,rounding_size=0.1',
                             facecolor=color, edgecolor='#2C3E50', linewidth=1.5)
        ax.add_patch(rect)
        ax.text(bx + 2.4, by + 1.4, name, ha='center', va='center',
                fontsize=11, fontweight='bold', color=C['white'])
        ax.text(bx + 2.4, by + 0.5, f'({desc})', ha='center', va='center',
                fontsize=9, color='#D5DBDB', style='italic')
    y -= 7.5

    # Arrow down
    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === LAYER 5: INTELLIGENCE ===
    draw_rect(ax, 0.5, y-4.5, 21, 4, C['intelligence'], 'agent-mem-intelligence', None, 16)
    draw_status_badge(ax, 18.5, y-0.5, 'COMPLETE', 9)
    components = ['Fact\nExtraction', 'Conflict\nResolution', 'Decision\nEngine', 'Importance\nScorer',
                  'Forgetting\nEngine', 'Metacognition', 'Auto-\nConsolidation', 'Reranking']
    for i, comp in enumerate(components):
        col = i % 4
        row = i // 4
        bx = 1 + col * 5.2
        by = y - 3.8 - row * 1.8
        rect = FancyBboxPatch(xy=(bx, by), width=4.8, height=1.5,
                             boxstyle='round,pad=0.01,rounding_size=0.08',
                             facecolor=C['white'], edgecolor='none', alpha=0.95)
        ax.add_patch(rect)
        ax.text(bx + 2.4, by + 0.75, comp, ha='center', va='center',
                fontsize=9, fontweight='bold', color=C['intelligence'])
    y -= 6

    # Arrow down
    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === LAYER 6: STORAGE ===
    draw_rect(ax, 0.3, y-8, 21.4, 7.5, C['storage'], 'StorageFactory (14+ Backends)', None, 16)
    draw_status_badge(ax, 18.5, y-0.5, 'COMPLETE', 9)

    # L1 Cache
    draw_rect(ax, 0.8, y-4, 4.5, 3.5, C['cache'], 'L1 Cache', 'In-Memory LRU', 12)
    # L2 Cache
    draw_rect(ax, 0.8, y-7, 4.5, 2.8, C['cache'], 'L2 Cache', 'Redis', 12)

    # Structured Storage
    draw_rect(ax, 6, y-7.5, 7.5, 7, C['structured'], 'Structured Storage', None, 13)
    for i, s in enumerate(['LibSQL', 'PostgreSQL', 'MongoDB', 'MySQL', 'Redis']):
        ax.text(6.5, y - 1.5 - i * 1.2, f'● {s}', ha='left', va='center',
                fontsize=10, fontweight='bold', color=C['white'])

    # Vector Storage
    draw_rect(ax, 14.5, y-7.5, 7, 7, C['vector'], 'Vector Storage', None, 13)
    vecs = ['LanceDB', 'Qdrant', 'Pinecone', 'FAISS', 'pgvector', 'Chroma', 'Weaviate', 'Milvus', 'Supabase']
    for i, v in enumerate(vecs):
        col = i % 2
        row = i // 2
        ax.text(15, y - 1.5 - row * 1.4, f'● {v}', ha='left', va='center',
                fontsize=9, fontweight='bold', color=C['white'])

    y -= 10

    # Arrow down
    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === LAYER 7: UI (CRITICAL GAP) ===
    draw_rect(ax, 0.3, y-12, 21.4, 11.5, C['client'], 'UI LAYER (Next.js) - ⚠️ CRITICAL GAPS', None, 18)
    draw_status_badge(ax, 18.5, y-0.5, 'PARTIAL', 9)

    # Show UI completeness
    ui_items = [
        ('/ (Homepage)', 'COMPLETE', '54KB full page'),
        ('/admin (Dashboard)', 'COMPLETE', 'Real stats connected'),
        ('/admin/agents', 'PARTIAL', 'Basic list view'),
        ('/admin/chat', 'PLACEHOLDER', 'No SSE streaming'),
        ('/admin/memories', 'PARTIAL', 'Search not complete'),
        ('/admin/graph', 'PARTIAL', '27KB static'),
        ('/admin/plugins', 'PLACEHOLDER', 'No WASM UI'),
        ('/admin/users', 'COMPLETE', 'User cards'),
        ('/admin/settings', 'PLACEHOLDER', 'No real config'),
        ('File-Centric UI', 'MISSING', 'No resource mgmt'),
        ('Working Memory UI', 'MISSING', 'No session view'),
        ('MCP Tools UI', 'MISSING', 'No tool list'),
        ('Log Aggregation UI', 'MISSING', 'No query view'),
        ('Performance UI', 'MISSING', 'No charts'),
    ]

    for i, (name, status, desc) in enumerate(ui_items):
        col = i % 2
        row = i // 2
        bx = 0.8 + col * 10.5
        by = y - 2 - row * 1.5

        status_colors = {
            'COMPLETE': C['complete'],
            'PARTIAL': C['partial'],
            'MISSING': C['missing'],
            'PLACEHOLDER': C['placeholder']
        }
        status_symbols = {
            'COMPLETE': '[OK]',
            'PARTIAL': '[!]',
            'MISSING': '[X]',
            'PLACEHOLDER': '[.]'
        }

        rect = FancyBboxPatch(xy=(bx, by), width=10, height=1.3,
                             boxstyle='round,pad=0.01,rounding_size=0.05',
                             facecolor=status_colors[status], edgecolor='none', alpha=0.7)
        ax.add_patch(rect)
        ax.text(bx + 0.3, by + 0.65, f'{status_symbols[status]} {name}', ha='left', va='center',
                fontsize=9, fontweight='bold', color=C['white'])
        ax.text(bx + 0.3, by + 0.2, desc, ha='left', va='center',
                fontsize=7, color='#D5DBDB', style='italic')

    # Gap summary
    ax.text(11, y - 11, '[X] CRITICAL: 6 pages missing | [!] PARTIAL: 3 pages incomplete | [.] PLACEHOLDER: 3 pages need work',
             ha='center', fontsize=10, color='#C0392B', fontweight='bold')

    plt.tight_layout()
    return fig


# ============================================================================
# UI-ARCHITECTURE GAP ANALYSIS
# ============================================================================
def create_ui_gap_analysis():
    fig, ax = plt.subplots(figsize=(20, 24))
    ax.set_xlim(0, 20)
    ax.set_ylim(0, 24)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    # Title
    ax.text(10, 23.2, 'UI-Backend Integration Gap Analysis', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(10, 22.6, 'Frontend vs Backend Completeness Comparison', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    y = 21.5

    # Backend Status (Top Row)
    draw_rect(ax, 0.5, y-3, 19, 2.8, C['complete'], 'Backend (Rust) - 100% Complete', None, 16)
    backend_apis = [
        ('Memory CRUD', '[OK]'), ('Agents', '[OK]'), ('Chat+SSE', '[OK]'), ('File-Centric', '[OK]'),
        ('Working Memory', '[OK]'), ('MCP Server', '[OK]'), ('Plugins', '[OK]'), ('Stats API', '[OK]'),
        ('Logs', '[OK]'), ('Performance', '[OK]'), ('WebSocket', '[OK]'), ('Organizations', '[OK]')
    ]
    for i, (name, status) in enumerate(backend_apis):
        col = i % 6
        row = i // 6
        bx = 1 + col * 3.1
        by = y - 2.5 - row * 1
        ax.text(bx, by + 0.2, f'{status} {name}', fontsize=9, color=C['white'], va='center')
    y -= 4.5

    # Gap visualization
    draw_rect(ax, 0.5, y-8, 19, 7.5, C['missing'], 'Frontend (Next.js) - ~40% Complete', None, 18)

    # Gap analysis table
    gaps = [
        ('File-Centric UI', '51KB backend', '0KB frontend', 'MISSING', C['missing']),
        ('Working Memory UI', '10KB backend', '0KB frontend', 'MISSING', C['missing']),
        ('MCP Tools UI', '7KB backend', '0KB frontend', 'MISSING', C['missing']),
        ('Log Aggregation UI', '17KB backend', '0KB frontend', 'MISSING', C['missing']),
        ('Performance Monitor UI', '13KB backend', '0KB frontend', 'MISSING', C['missing']),
        ('Chat SSE Streaming', '16KB backend', '3KB frontend', 'PARTIAL', C['partial']),
        ('Knowledge Graph UI', '10KB backend', '27KB static', 'PARTIAL', C['partial']),
        ('Plugins Management', '12KB backend', '24KB static', 'PARTIAL', C['partial']),
        ('Settings Page', '0KB backend', '4KB placeholder', 'PLACEHOLDER', C['placeholder']),
    ]

    for i, (feature, backend, frontend, status, color) in enumerate(gaps):
        row = i // 3
        col = i % 3
        bx = 0.8 + col * 6.3
        by = y - 1.5 - row * 2.2

        rect = FancyBboxPatch(xy=(bx, by), width=6, height=2,
                             boxstyle='round,pad=0.02,rounding_size=0.1',
                             facecolor=color, edgecolor='none', alpha=0.8)
        ax.add_patch(rect)

        ax.text(bx + 3, by + 1.5, feature, ha='center', fontsize=10,
                fontweight='bold', color=C['white'])
        ax.text(bx + 3, by + 0.9, f'Backend: {backend}', ha='center', fontsize=8, color='#D5DBDB')
        ax.text(bx + 3, by + 0.4, f'Frontend: {frontend}', ha='center', fontsize=8, color='#D5DBDB')
        ax.text(bx + 3, by + 0.0, f'Status: {status}', ha='center', fontsize=8, color=C['white'], fontweight='bold')
    y -= 10

    # Recommendations
    draw_rect(ax, 0.5, y-5, 19, 4.5, C['unified'], 'Priority Recommendations', None, 16)

    priorities = [
        ('P0', 'File-Centric UI', 'High value, 51KB backend ready'),
        ('P0', 'Chat SSE Streaming', 'Core UX, user experience'),
        ('P1', 'Working Memory UI', 'Session context visualization'),
        ('P1', 'MCP Tools UI', 'Tool management interface'),
        ('P2', 'Performance Monitor', 'System health dashboard'),
        ('P2', 'Log Aggregation UI', 'Debugging interface'),
    ]

    for i, (priority, feature, desc) in enumerate(priorities):
        col = i % 2
        row = i // 2
        bx = 0.8 + col * 9.5
        by = y - 1.5 - row * 1.3

        priority_color = {'P0': C['missing'], 'P1': C['partial'], 'P2': C['client']}[priority]

        draw_chip(ax, bx, by, 0.8, 0.9, priority, priority_color, 9)
        ax.text(bx + 1, by + 0.45, f'{feature} - {desc}', fontsize=9, color=C['white'], va='center')

    plt.tight_layout()
    return fig


# ============================================================================
# COMPLETE FLOW WITH UI INTEGRATION
# ============================================================================
def create_complete_flow():
    fig, ax = plt.subplots(figsize=(20, 28))
    ax.set_xlim(0, 20)
    ax.set_ylim(0, 28)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    # Title
    ax.text(10, 27.2, 'Complete System Flow with UI Integration', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(10, 26.6, 'Showing Frontend-Backend Data Flow', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    y = 25.5

    # === CLIENT LAYER ===
    draw_rect(ax, 0.5, y-3.5, 19, 3, C['client'], 'Client Layer', None, 16)

    # Client types
    clients = [
        ('Web Browser', 'Next.js UI', '[OK]'),
        ('Mobile App', 'React Native', '[.]'),
        ('Python App', 'Python SDK', '[OK]'),
        ('CLI Tool', 'TypeScript SDK', '[OK]'),
    ]
    for i, (name, tech, status) in enumerate(clients):
        bx = 1 + i * 4.7
        by = y - 3
        draw_rect(ax, bx, by, 4.3, 2.5, C['client'], name, tech, 11)
        ax.text(bx + 2.15, by + 0.3, status, ha='center', fontsize=10, color=C['white'])
    y -= 5

    # Arrow with label
    draw_arrow(ax, 10, y, 10, y - 0.8)
    ax.text(10.5, y - 0.4, 'HTTP/WebSocket', fontsize=8, color='#7F8C8D')
    y -= 1.2

    # === API GATEWAY ===
    draw_rect(ax, 0.5, y-4, 19, 3.5, C['api'], 'API Gateway (Axum)', '100+ REST Endpoints | WebSocket | SSE', 16)
    draw_status_badge(ax, 17, y-0.5, 'COMPLETE', 9)

    routes = [
        ('/api/v1/memories', '[OK]', 'Complete'),
        ('/api/v1/agents', '[OK]', 'Complete'),
        ('/api/v1/file-centric/*', '[OK]', 'Complete'),
        ('/api/v1/stats/*', '[OK]', 'Complete'),
        ('/api/v1/working-memory', '[OK]', 'Complete'),
        ('/api/v1/mcp/*', '[OK]', 'Complete'),
    ]
    for i, (route, status, desc) in enumerate(routes):
        col = i % 3
        row = i // 3
        bx = 1 + col * 6.2
        by = y - 1.5 - row * 1.2
        ax.text(bx, by + 0.2, f'{status} {route}', fontsize=8, color=C['white'], family='monospace')
    y -= 6

    # Arrow
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === UNIFIED API ===
    draw_rect(ax, 2, y-3, 16, 2.5, C['unified'], 'Unified Memory API (agent-mem)', None, 14)
    draw_status_badge(ax, 15.5, y-0.5, 'COMPLETE', 9)
    methods = ['Memory::add()', 'Memory::get()', 'Memory::search()', 'Memory::update()', 'Memory::delete()']
    for i, m in enumerate(methods):
        draw_chip(ax, 3 + i * 3, y - 2.5, 2.8, 0.7, m, C['white'], 8, C['unified'])
    y -= 5

    # Arrow
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === ORCHESTRATION ===
    draw_rect(ax, 0.5, y-5, 19, 4.5, C['orchestration'], 'MemoryEngine (8 Agents)', None, 16)
    draw_status_badge(ax, 17, y-0.5, 'COMPLETE', 9)

    agents = [
        ('CoreAgent', C['core']), ('SemanticAgent', C['semantic']),
        ('EpisodicAgent', C['episodic']), ('WorkingAgent', C['working']),
        ('ProceduralAgent', C['procedural']), ('KnowledgeAgent', C['knowledge']),
        ('ResourceAgent', C['resource']), ('ContextualAgent', C['contextual']),
    ]
    for i, (name, color) in enumerate(agents):
        row = i // 4
        col = i % 4
        bx = 1 + col * 4.7
        by = y - 4.5 - row * 2
        draw_rect(ax, bx, by, 4.3, 1.8, color, name, None, 11)
    y -= 7

    # Arrow
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === INTELLIGENCE LAYER ===
    draw_rect(ax, 0.5, y-3.5, 19, 3, C['intelligence'], 'Intelligence Layer', None, 16)
    draw_status_badge(ax, 17, y-0.5, 'COMPLETE', 9)
    components = ['Fact Extraction', 'Conflict Resolution', 'Decision Engine', 'Importance Scorer', 'Forgetting Engine']
    for i, c in enumerate(components):
        draw_chip(ax, 1 + i * 3.7, y - 2.8, 3.5, 0.8, c, C['white'], 9, C['intelligence'])
    y -= 5

    # Arrow
    draw_arrow(ax, 10, y, 10, y - 0.8)
    y -= 1.2

    # === STORAGE ===
    draw_rect(ax, 0.5, y-4.5, 9, 4, C['structured'], 'Structured Storage', None, 14)
    draw_rect(ax, 10.5, y-4.5, 9, 4, C['vector'], 'Vector Storage', None, 14)

    structured = ['LibSQL', 'PostgreSQL', 'MongoDB']
    vector = ['LanceDB', 'Qdrant', 'Chroma', 'FAISS']
    for i, s in enumerate(structured):
        ax.text(1.5, y - 1.5 - i * 1, f'● {s}', fontsize=10, color=C['white'])
    for i, v in enumerate(vector):
        ax.text(11.5, y - 1.5 - i * 1, f'● {v}', fontsize=10, color=C['white'])

    plt.tight_layout()
    return fig


# ============================================================================
# MAIN
# ============================================================================
def main():
    output_dir = '/Users/louloulin/Documents/linchong/cjproject/contextengine/agentmen'

    diagrams = [
        ('arch101_main.png', create_main, 'Main Architecture with UI Status'),
        ('arch101_ui_gap.png', create_ui_gap_analysis, 'UI-Backend Gap Analysis'),
        ('arch101_complete_flow.png', create_complete_flow, 'Complete System Flow'),
    ]

    print("Generating architecture diagrams with UI analysis...")
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
