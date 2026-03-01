# Scratchpad - Cleanup Intermediate Files

## Objective
Clean up unnecessary intermediate process files

## Understanding
Based on analysis:
1. **Temp directory cleanup**: `/var/folders/nj/vtk9xv2j4wq41_94ry3zr8hh0000gn/T/` contains numerous .tmp* files
2. **Project backup files**: Found `.bak` file at `sdks/cangjie/src/core/errors.cj.bak`
3. **Log files**: Multiple log files in `logs/` and `logs/archived/` directories
4. **Root level logs**: `backend.log` and `frontend.log` in project root

## Approach
Phase 1: Identify and categorize cleanup targets
- System temp files (.tmp*)
- Project backup files (.bak)
- Old archived logs (safe to remove if old)
- Root-level log files (should be in logs/ directory)

Phase 2: Create tasks for each cleanup category
Phase 3: Execute cleanup systematically with verification

## Safety Considerations
- Only remove files that are clearly temporary or backup
- Preserve current active logs
- Keep .ralph directory intact (it's active runtime state)
- Don't remove build artifacts or project data
