#!/usr/bin/env python3
"""
AgentMem Architecture Diagram Generator - ACCURATE UI STATUS
Shows REAL implementation status based on actual code analysis
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
    'complete': '#27AE60',
    'partial': '#F39C12',
    'missing': '#C0392B',
    'placeholder': '#95A5A6',
}


def draw_rect(ax, x, y, w, h, color, title=None, subtitle=None, fontsize=11, title_color=None):
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
    if text_color is None:
        text_color = C['white']
    rect = FancyBboxPatch(xy=(x, y), width=w, height=h,
                         boxstyle='round,pad=0.015,rounding_size=0.1',
                         facecolor=color, edgecolor='none', alpha=0.95)
    ax.add_patch(rect)
    ax.text(x + w/2, y + h/2, text, ha='center', va='center',
            fontsize=fontsize, fontweight='bold', color=text_color)


def draw_arrow(ax, x1, y1, x2, y2, color='#85929E'):
    ax.annotate('', xy=(x2, y2), xytext=(x1, y1),
                arrowprops=dict(arrowstyle='->', color=color, lw=2.5,
                              connectionstyle='arc3,rad=0'),
                zorder=2)


# ============================================================================
# UI COMPLETENESS ANALYSIS (Based on actual code)
# ============================================================================
def create_ui_status():
    """Show REAL UI implementation status"""
    fig, ax = plt.subplots(figsize=(22, 20))
    ax.set_xlim(0, 22)
    ax.set_ylim(0, 20)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    # Title
    ax.text(11, 19.3, 'AgentMem UI Implementation Status', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(11, 18.7, 'Based on Actual Code Analysis (May 2026)', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    y = 17.5

    # === BACKEND STATUS ===
    draw_rect(ax, 0.5, y-3, 21, 2.5, C['complete'], 'Backend (Rust) - Production Ready', None, 16)

    backend_items = [
        ('100+ REST API Routes', '[OK]'),
        ('8 Memory Agents', '[OK]'),
        ('File-Centric API (51KB)', '[OK]'),
        ('Working Memory API', '[OK]'),
        ('MCP Server', '[OK]'),
        ('WebSocket/SSE', '[OK]'),
        ('14+ Storage Backends', '[OK]'),
        ('Stats & Metrics', '[OK]'),
    ]
    for i, (name, status) in enumerate(backend_items):
        col = i % 4
        row = i // 4
        bx = 1 + col * 5.2
        by = y - 2.2 - row * 1
        ax.text(bx, by, f'{status} {name}', fontsize=10, color=C['white'])
    y -= 4

    # === FRONTEND STATUS ===
    draw_rect(ax, 0.5, y-12, 21, 11.5, C['client'], 'Frontend (Next.js) - Implementation Status', None, 18)

    # Actual implemented pages
    implemented = [
        ('/ (Homepage)', '[OK] COMPLETE', '54KB - Full marketing page with animations', C['complete']),
        ('/admin (Dashboard)', '[OK] COMPLETE', 'Real stats from API, WebSocket status', C['complete']),
        ('/admin/chat', '[OK] COMPLETE', '750 lines - SSE streaming, LumosAI, memory panel', C['complete']),
        ('/admin/memories', '[OK] COMPLETE', '500+ lines - Table view, pagination, filtering', C['complete']),
        ('/admin/graph', '[OK] COMPLETE', '500+ lines - Force-directed layout, interactive', C['complete']),
        ('/admin/agents', '[OK] COMPLETE', 'Agent CRUD, state management', C['complete']),
        ('/admin/plugins', '[OK] COMPLETE', '623 lines - WASM upload, details dialog', C['complete']),
        ('/admin/users', '[OK] COMPLETE', 'User management, cards display', C['complete']),
        ('/admin/settings', '[OK] PARTIAL', 'Basic settings form (129 lines)', C['partial']),
    ]

    # Missing pages
    missing = [
        ('File-Centric UI', '[X] MISSING', 'No resource management page', C['missing']),
        ('Working Memory UI', '[X] MISSING', 'No session context visualization', C['missing']),
        ('Performance UI', '[X] MISSING', 'No system monitoring dashboard', C['missing']),
        ('Logs Query UI', '[X] MISSING', 'No log aggregation interface', C['missing']),
    ]

    # Draw implemented pages
    for i, (name, status, desc, color) in enumerate(implemented):
        row = i // 3
        col = i % 3
        bx = 0.8 + col * 7
        by = y - 1.2 - row * 1.8

        rect = FancyBboxPatch(xy=(bx, by), width=6.7, height=1.5,
                             boxstyle='round,pad=0.02,rounding_size=0.08',
                             facecolor=color, edgecolor='none', alpha=0.85)
        ax.add_patch(rect)
        ax.text(bx + 3.35, by + 1.1, name, ha='center', fontsize=11,
                fontweight='bold', color=C['white'])
        ax.text(bx + 3.35, by + 0.6, status, ha='center', fontsize=9,
                color=C['white'])
        ax.text(bx + 3.35, by + 0.15, desc, ha='center', fontsize=7,
                color='#D5DBDB', style='italic')

    y -= 8

    # Draw missing pages
    for i, (name, status, desc, color) in enumerate(missing):
        bx = 0.8 + i * 5.3
        by = y - 1.5

        rect = FancyBboxPatch(xy=(bx, by), width=5, height=1.3,
                             boxstyle='round,pad=0.02,rounding_size=0.08',
                             facecolor=color, edgecolor='none', alpha=0.85)
        ax.add_patch(rect)
        ax.text(bx + 2.5, by + 0.9, name, ha='center', fontsize=10,
                fontweight='bold', color=C['white'])
        ax.text(bx + 2.5, by + 0.35, status, ha='center', fontsize=9,
                color=C['white'])

    # Summary
    ax.text(11, 0.8, 'COMPLETE: 8 pages | PARTIAL: 1 page | MISSING: 4 pages',
             ha='center', fontsize=12, color='#2C3E50', fontweight='bold')

    plt.tight_layout()
    return fig


# ============================================================================
# COMPONENT DETAIL STATUS
# ============================================================================
def create_component_status():
    """Show detailed component implementation status"""
    fig, ax = plt.subplots(figsize=(20, 22))
    ax.set_xlim(0, 20)
    ax.set_ylim(0, 22)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(10, 21.3, 'Component Implementation Status', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(10, 20.7, 'Detailed analysis of UI components', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    y = 19.5

    # === CHAT COMPONENT ===
    draw_rect(ax, 0.5, y-4, 9.3, 3.5, C['complete'], 'Chat Component (/admin/chat)', None, 14)
    chat_features = [
        '[OK] SSE streaming response',
        '[OK] LumosAI integration',
        '[OK] Memory panel integration',
        '[OK] Agent selector',
        '[OK] Streaming/standard toggle',
        '[OK] Session management',
    ]
    for i, feat in enumerate(chat_features):
        ax.text(1, y - 1.5 - i * 0.45, feat, fontsize=9, color=C['white'])
    y -= 5

    # === MEMORIES COMPONENT ===
    draw_rect(ax, 0.5, y-4, 9.3, 3.5, C['complete'], 'Memories Component (/admin/memories)', None, 14)
    mem_features = [
        '[OK] Table view with pagination',
        '[OK] Advanced filtering/search',
        '[OK] Toast notifications',
        '[OK] Skeleton loading states',
        '[OK] Add/Edit/Delete memory',
        '[OK] Agent filtering',
    ]
    for i, feat in enumerate(mem_features):
        ax.text(1, y - 1.5 - i * 0.45, feat, fontsize=9, color=C['white'])
    y -= 5

    # === GRAPH COMPONENT ===
    draw_rect(ax, 0.5, y-4, 9.3, 3.5, C['complete'], 'Graph Component (/admin/graph)', None, 14)
    graph_features = [
        '[OK] Force-directed layout',
        '[OK] Interactive node dragging',
        '[OK] Node search & filter',
        '[OK] Relationship highlighting',
        '[OK] Graph analytics',
        '[OK] Zoom/pan controls',
    ]
    for i, feat in enumerate(graph_features):
        ax.text(1, y - 1.5 - i * 0.45, feat, fontsize=9, color=C['white'])
    y -= 5

    # === PLUGINS COMPONENT ===
    draw_rect(ax, 10.2, 14.5, 9.3, 3.5, C['complete'], 'Plugins Component (/admin/plugins)', None, 14)
    plugin_features = [
        '[OK] WASM file upload',
        '[OK] Plugin list view',
        '[OK] Details dialog',
        '[OK] Status icons',
        '[OK] Type badges',
        '[OK] Auto-refresh',
    ]
    for i, feat in enumerate(plugin_features):
        ax.text(10.7, 17 - 1.5 - i * 0.45, feat, fontsize=9, color=C['white'])

    # === AGENTS COMPONENT ===
    draw_rect(ax, 10.2, 9.5, 9.3, 3.5, C['complete'], 'Agents Component (/admin/agents)', None, 14)
    agent_features = [
        '[OK] Agent list view',
        '[OK] Create agent form',
        '[OK] State management',
        '[OK] Chat integration',
        '[OK] State indicators',
        '[OK] Delete functionality',
    ]
    for i, feat in enumerate(agent_features):
        ax.text(10.7, 12 - 1.5 - i * 0.45, feat, fontsize=9, color=C['white'])

    # === MISSING COMPONENTS ===
    draw_rect(ax, 0.5, 4, 19, 4, C['missing'], 'Missing Components - Need Implementation', None, 16)

    missing_components = [
        ('File-Centric UI', 'Resource mounting, category management, extraction pipeline'),
        ('Working Memory UI', 'Session context visualization, priority management'),
        ('Performance UI', 'System metrics, database pool stats, index performance'),
        ('Logs Query UI', 'Log aggregation, trace ID lookup, filtering'),
    ]
    for i, (name, desc) in enumerate(missing_components):
        col = i % 2
        row = i // 2
        bx = 0.8 + col * 9.5
        by = 4.8 - row * 1.8

        rect = FancyBboxPatch(xy=(bx, by), width=9, height=1.5,
                             boxstyle='round,pad=0.02,rounding_size=0.08',
                             facecolor=C['red'], edgecolor='none', alpha=0.7)
        ax.add_patch(rect)
        ax.text(bx + 4.5, by + 1, name, ha='center', fontsize=11,
                fontweight='bold', color=C['white'])
        ax.text(bx + 4.5, by + 0.3, desc, ha='center', fontsize=8,
                color='#D5DBDB', style='italic')

    # Summary
    ax.text(10, 0.8, 'UI Completeness: ~70% (8/12 components complete)',
             ha='center', fontsize=12, color='#2C3E50', fontweight='bold')

    plt.tight_layout()
    return fig


# ============================================================================
# ARCHITECTURE WITH UI INTEGRATION
# ============================================================================
def create_arch_with_ui():
    """Complete architecture showing UI integration points"""
    fig, ax = plt.subplots(figsize=(22, 26))
    ax.set_xlim(0, 22)
    ax.set_ylim(0, 26)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_facecolor(C['cream'])
    fig.patch.set_facecolor(C['cream'])

    ax.text(11, 25.3, 'AgentMem Complete Architecture', ha='center', fontsize=22,
            fontweight='bold', color='#2C3E50')
    ax.text(11, 24.7, 'Showing Frontend-Backend Integration', ha='center',
            fontsize=11, color='#7F8C8D', style='italic')

    y = 23.5

    # === CLIENT LAYER ===
    draw_rect(ax, 0.5, y-2.5, 21, 2, C['client'], 'Client Layer', None, 16)
    clients = [
        ('Web Browser', 'Next.js UI', '[OK]'),
        ('Python App', 'Python SDK', '[OK]'),
        ('Go App', 'Go SDK', '[OK]'),
        ('CLI Tool', 'TS SDK', '[OK]'),
    ]
    for i, (name, tech, status) in enumerate(clients):
        bx = 1 + i * 5.2
        by = y - 2.2
        draw_rect(ax, bx, by, 4.8, 1.8, C['client'], name, tech, 11)
        ax.text(bx + 2.4, by + 0.3, status, ha='center', fontsize=9, color=C['white'])
    y -= 3.5

    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === UI PAGES ===
    draw_rect(ax, 0.5, y-5, 21, 4.5, C['client'], 'UI Layer (Next.js) - 8 Pages Implemented', None, 16)

    pages = [
        ('/ (Home)', C['complete']), ('/admin', C['complete']),
        ('/admin/chat', C['complete']), ('/admin/memories', C['complete']),
        ('/admin/graph', C['complete']), ('/admin/agents', C['complete']),
        ('/admin/plugins', C['complete']), ('/admin/users', C['complete']),
    ]
    for i, (page, color) in enumerate(pages):
        col = i % 4
        row = i // 4
        bx = 1 + col * 5.2
        by = y - 1.5 - row * 1.5
        draw_chip(ax, bx, by, 4.8, 1.2, page, color, 10)
    y -= 6.5

    # === API GATEWAY ===
    draw_rect(ax, 0.5, y-3.5, 21, 3, C['api'], 'API Gateway (Axum) - 100+ Routes', None, 16)

    routes = [
        ('/api/v1/memories', '[OK]'),
        ('/api/v1/agents', '[OK]'),
        ('/api/v1/file-centric/*', '[OK]'),
        ('/api/v1/stats/*', '[OK]'),
        ('/api/v1/working-memory', '[OK]'),
        ('/api/v1/mcp/*', '[OK]'),
        ('/api/v1/plugins', '[OK]'),
        ('/api/v1/logs/*', '[OK]'),
    ]
    for i, (route, status) in enumerate(routes):
        col = i % 4
        row = i // 4
        bx = 1 + col * 5.2
        by = y - 1.5 - row * 1.2
        ax.text(bx, by, f'{status} {route}', fontsize=8, color=C['white'], family='monospace')
    y -= 5

    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === UNIFIED API ===
    draw_rect(ax, 2, y-2.5, 18, 2, C['unified'], 'Unified Memory API', None, 14)
    for i, m in enumerate(['Memory::add()', 'Memory::get()', 'Memory::search()', 'Memory::update()', 'Memory::delete()']):
        draw_chip(ax, 3 + i * 3.5, y - 2.2, 3.2, 0.7, m, C['white'], 8, C['unified'])
    y -= 4

    draw_arrow(ax, 11, y, 11, y - 0.8)
    y -= 1.2

    # === ORCHESTRATION ===
    draw_rect(ax, 0.5, y-4, 21, 3.5, C['orchestration'], 'MemoryEngine (8 Agents)', None, 16)

    agents = [
        ('CoreAgent', C['core']), ('SemanticAgent', C['semantic']),
        ('EpisodicAgent', C['episodic']), ('WorkingAgent', C['working']),
        ('ProceduralAgent', C['procedural']), ('KnowledgeAgent', C['knowledge']),
        ('ResourceAgent', C['resource']), ('ContextualAgent', C['contextual']),
    ]
    for i, (name, color) in enumerate(agents):
        row = i // 4
        col = i % 4
        bx = 1 + col * 5.2
        by = y - 3.8 - row * 1.8
        draw_chip(ax, bx, by, 4.8, 1.5, name, color, 10)
    y -= 5.5

    # === STORAGE ===
    draw_rect(ax, 0.5, 0.5, 10, 2.5, C['structured'], 'Structured Storage', None, 12)
    for i, s in enumerate(['LibSQL', 'PostgreSQL', 'MongoDB', 'Redis']):
        ax.text(1.2, 2.5 - i * 0.5, f'- {s}', fontsize=9, color=C['white'])

    draw_rect(ax, 11.5, 0.5, 10, 2.5, C['vector'], 'Vector Storage', None, 12)
    for i, v in enumerate(['LanceDB', 'Qdrant', 'Chroma', 'FAISS', 'Pinecone']):
        ax.text(12.2, 2.5 - i * 0.5, f'- {v}', fontsize=9, color=C['white'])

    plt.tight_layout()
    return fig


# ============================================================================
# MAIN
# ============================================================================
def main():
    output_dir = '/Users/louloulin/Documents/linchong/cjproject/contextengine/agentmen'

    diagrams = [
        ('arch101_ui_status.png', create_ui_status, 'UI Implementation Status'),
        ('arch101_component_status.png', create_component_status, 'Component Status'),
        ('arch101_arch_with_ui.png', create_arch_with_ui, 'Architecture with UI'),
    ]

    print("Generating accurate architecture diagrams...")
    for filename, func, desc in diagrams:
        try:
            fig = func()
            fig.savefig(f'{output_dir}/{filename}', dpi=150, bbox_inches='tight',
                       facecolor=C['cream'], edgecolor='none')
            plt.close(fig)
            print(f"  OK: {filename} ({desc})")
        except Exception as e:
            print(f"  FAIL: {filename}: {e}")
            import traceback
            traceback.print_exc()

    print("\nDone!")


if __name__ == '__main__':
    main()
